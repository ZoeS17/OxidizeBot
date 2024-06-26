use anyhow::Result;
use async_trait::async_trait;
use chat::command;
use chat::module;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use tokio::sync::Mutex;

/// Handler for the !poll command.
pub(crate) struct Poll {
    enabled: settings::Var<bool>,
    polls: Mutex<HashMap<command::HookId, ActivePoll>>,
}

#[async_trait]
impl command::Handler for Poll {
    fn scope(&self) -> Option<auth::Scope> {
        Some(auth::Scope::Poll)
    }

    async fn handle(&self, ctx: &mut command::Context<'_>) -> Result<()> {
        if !self.enabled.load().await {
            return Ok(());
        }

        match ctx.next().as_deref() {
            Some("run") => {
                let question = ctx.next_str("<question> <options...>")?;

                let mut options = HashMap::new();

                for option in ctx.by_ref() {
                    let (key, description) = match option.find('=') {
                        Some(i) => {
                            let (keyword, description) = option.split_at(i);
                            (keyword.to_string(), Some(description[1..].to_string()))
                        }
                        None => (option, None),
                    };

                    options.insert(key.to_lowercase(), description);
                }

                let poll = ActivePoll {
                    question: question.clone(),
                    created_at: Utc::now(),
                    options,
                    inner: settings::Var::new(Inner {
                        voted: Default::default(),
                        votes: Default::default(),
                    }),
                };

                let hook_id = ctx.insert_hook(poll.clone()).await;
                self.polls.lock().await.insert(hook_id, poll);
                ctx.respond(format!("Started poll `{}` (id: {})", question, hook_id))
                    .await;
            }
            Some("close") => {
                let mut polls = self.polls.lock().await;

                let id = match ctx.next() {
                    Some(id) => str::parse::<command::HookId>(&id)
                        .map_err(|_| chat::respond_err!("Bad id `{}`", id))?,
                    None => {
                        *polls
                            .iter()
                            .max_by_key(|e| e.1.created_at)
                            .ok_or(chat::respond_err!("No running polls"))?
                            .0
                    }
                };

                let poll = polls
                    .remove(&id)
                    .ok_or(chat::respond_err!("No poll with id `{}`!", id))?;

                ctx.remove_hook(id).await;
                let results = poll.close().await;

                let total = results.iter().map(|(_, c)| c).sum::<u32>();

                let mut formatted = Vec::new();

                for (key, votes) in results {
                    let p = common::percentage(votes, total);

                    let votes = match votes {
                        0 => "no votes".to_string(),
                        1 => "one vote".to_string(),
                        n => format!("{} votes", n),
                    };

                    formatted.push(format!("{} = {} ({})", key, votes, p));
                }

                chat::respond!(ctx, "{} -> {}.", poll.question, formatted.join(", "));
            }
            _ => {
                ctx.respond("Expected: run, close.").await;
            }
        }

        Ok(())
    }
}

struct Inner {
    voted: HashSet<String>,
    votes: HashMap<String, u32>,
}

#[derive(Clone)]
struct ActivePoll {
    question: String,
    created_at: DateTime<Utc>,
    options: HashMap<String, Option<String>>,
    inner: settings::Var<Inner>,
}

impl ActivePoll {
    /// Close the poll.
    pub(crate) async fn close(&self) -> Vec<(String, u32)> {
        let inner = self.inner.read().await;

        let mut results = Vec::new();

        for (o, description) in &self.options {
            results.push((
                description.clone().unwrap_or_else(|| o.to_string()),
                inner.votes.get(o).cloned().unwrap_or_default(),
            ));
        }

        results.sort_by(|a, b| b.1.cmp(&a.1));
        results
    }
}

#[async_trait]
impl command::MessageHook for ActivePoll {
    async fn peek(&self, user: &chat::User, m: &str) -> Result<()> {
        let mut inner = self.inner.write().await;

        let user = match user.real() {
            Some(user) => user,
            None => return Ok(()),
        };

        if inner.voted.contains(user.login()) {
            return Ok(());
        }

        for word in common::words::trimmed(m) {
            if !self.options.contains_key(&word.to_lowercase()) {
                continue;
            }

            *inner.votes.entry(word.to_string()).or_default() += 1;
            inner.voted.insert(user.login().to_string());
            break;
        }

        Ok(())
    }
}

pub(crate) struct Module;

#[async_trait]
impl chat::Module for Module {
    fn ty(&self) -> &'static str {
        "poll"
    }

    /// Set up command handlers for this module.
    async fn hook(
        &self,
        module::HookContext {
            handlers, settings, ..
        }: module::HookContext<'_, '_>,
    ) -> Result<()> {
        handlers.insert(
            "poll",
            Poll {
                polls: Mutex::new(Default::default()),
                enabled: settings.var("poll/enabled", false).await?,
            },
        );

        Ok(())
    }
}
