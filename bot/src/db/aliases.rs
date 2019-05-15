use crate::{db, template, utils};
use diesel::prelude::*;
use hashbrown::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;

/// Local database wrapper.
#[derive(Clone)]
struct Database(db::Database);

impl Database {
    private_database_group_fns!(aliases, Alias, Key);

    fn edit(&self, key: &Key, text: &str) -> Result<Option<db::models::Alias>, failure::Error> {
        use db::schema::aliases::dsl;
        let c = self.0.pool.lock();

        let filter =
            dsl::aliases.filter(dsl::channel.eq(&key.channel).and(dsl::name.eq(&key.name)));

        let first = filter.clone().first::<db::models::Alias>(&*c).optional()?;

        match first {
            None => {
                let alias = db::models::Alias {
                    channel: key.channel.to_string(),
                    name: key.name.to_string(),
                    text: text.to_string(),
                    group: None,
                    disabled: false,
                };

                diesel::insert_into(dsl::aliases)
                    .values(&alias)
                    .execute(&*c)?;
                Ok(Some(alias))
            }
            Some(alias) => {
                let mut set = db::models::UpdateAlias::default();
                set.text = Some(text);
                diesel::update(filter).set(&set).execute(&*c)?;

                if alias.disabled {
                    return Ok(None);
                }

                Ok(Some(alias))
            }
        }
    }

    fn delete(&self, key: &Key) -> Result<bool, failure::Error> {
        use db::schema::aliases::dsl;

        let c = self.0.pool.lock();
        let count = diesel::delete(
            dsl::aliases.filter(dsl::channel.eq(&key.channel).and(dsl::name.eq(&key.name))),
        )
        .execute(&*c)?;

        Ok(count == 1)
    }

    fn rename(&self, from: &Key, to: &Key) -> Result<bool, failure::Error> {
        use db::schema::aliases::dsl;

        let c = self.0.pool.lock();
        let count = diesel::update(
            dsl::aliases.filter(dsl::channel.eq(&from.channel).and(dsl::name.eq(&from.name))),
        )
        .set((dsl::name.eq(&to.name), dsl::name.eq(&to.channel)))
        .execute(&*c)?;

        Ok(count == 1)
    }
}

#[derive(Clone)]
pub struct Aliases {
    inner: Arc<RwLock<HashMap<Key, Arc<Alias>>>>,
    db: Database,
}

impl Aliases {
    database_group_fns!(Alias, Key);

    /// Construct a new commands store with a db.
    pub fn load(db: db::Database) -> Result<Aliases, failure::Error> {
        let mut inner = HashMap::new();

        let db = Database(db);

        for alias in db.list()? {
            let alias = Alias::from_db(alias)?;
            inner.insert(alias.key.clone(), Arc::new(alias));
        }

        Ok(Aliases {
            inner: Arc::new(RwLock::new(inner)),
            db,
        })
    }

    /// Lookup an alias based on a command prefix.
    pub fn lookup<'a>(&self, channel: &str, it: utils::Words<'a>) -> Option<(&'a str, String)> {
        let it = it.into_iter();

        let inner = self.inner.read();

        for (key, alias) in inner.iter() {
            if key.channel != channel {
                continue;
            }

            if let Some((m, out)) = alias.matches(it.clone()) {
                return Some((m, out));
            }
        }

        None
    }

    /// Insert a word into the bad words list.
    pub fn edit(
        &self,
        channel: &str,
        name: &str,
        template: template::Template,
    ) -> Result<(), failure::Error> {
        let key = Key::new(channel, name);

        let mut inner = self.inner.write();

        if let Some(alias) = self.db.edit(&key, template.source())? {
            log::info!("inserting alias in-memory");

            inner.insert(
                key.clone(),
                Arc::new(Alias {
                    key,
                    template,
                    group: alias.group,
                    disabled: alias.disabled,
                }),
            );
        } else {
            inner.remove(&key);
        }

        Ok(())
    }

    /// Remove alias.
    pub fn delete(&self, channel: &str, name: &str) -> Result<bool, failure::Error> {
        let key = Key::new(channel, name);

        if !self.db.delete(&key)? {
            return Ok(false);
        }

        self.inner.write().remove(&key);
        Ok(true)
    }

    /// Test the given word.
    pub fn get<'a>(&'a self, channel: &str, name: &str) -> Option<Arc<Alias>> {
        let key = Key::new(channel, name);

        let inner = self.inner.read();

        if let Some(alias) = inner.get(&key) {
            return Some(Arc::clone(alias));
        }

        None
    }

    /// Get a list of all commands.
    pub fn list(&self, channel: &str) -> Vec<Arc<Alias>> {
        let inner = self.inner.read();

        let mut out = Vec::new();

        for c in inner.values() {
            if c.key.channel != channel {
                continue;
            }

            out.push(Arc::clone(c));
        }

        out
    }

    /// Try to rename the alias.
    pub fn rename(&self, channel: &str, from: &str, to: &str) -> Result<(), super::RenameError> {
        let from_key = Key::new(channel, from);
        let to_key = Key::new(channel, to);

        let mut inner = self.inner.write();

        if inner.contains_key(&to_key) {
            return Err(super::RenameError::Conflict);
        }

        let alias = match inner.remove(&from_key) {
            Some(alias) => alias,
            None => return Err(super::RenameError::Missing),
        };

        let mut alias = (*alias).clone();
        alias.key = to_key.clone();

        match self.db.rename(&from_key, &to_key) {
            Err(e) => {
                log::error!("failed to rename alias `{}` in database: {}", from, e);
            }
            Ok(false) => {
                log::warn!("alias {} not renamed in database", from);
            }
            Ok(true) => (),
        }

        inner.insert(to_key, Arc::new(alias));
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct Key {
    pub channel: String,
    pub name: String,
}

impl Key {
    pub fn new(channel: &str, name: &str) -> Self {
        Self {
            channel: channel.to_string(),
            name: name.to_lowercase(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Alias {
    pub key: Key,
    pub template: template::Template,
    pub group: Option<String>,
    pub disabled: bool,
}

impl Alias {
    /// Convert a database alias into an in-memory alias.
    pub fn from_db(alias: db::models::Alias) -> Result<Alias, failure::Error> {
        let key = Key::new(alias.channel.as_str(), alias.name.as_str());
        let template = template::Template::compile(alias.text)?;

        Ok(Alias {
            key,
            template,
            group: alias.group,
            disabled: alias.disabled,
        })
    }

    /// Test if the given input matches and return the corresonding replacement if it does.
    pub fn matches<'a>(&self, mut it: utils::Words<'a>) -> Option<(&'a str, String)> {
        match it.next() {
            Some(value) if value.to_lowercase() == self.key.name => {
                let data = Data { rest: it.rest() };

                match self.template.render_to_string(&data) {
                    Ok(s) => return Some((value, s)),
                    Err(e) => {
                        log::error!("failed to render alias: {}", e);
                    }
                }
            }
            _ => {}
        }

        return None;

        #[derive(serde::Serialize)]
        struct Data<'a> {
            rest: &'a str,
        }
    }
}
