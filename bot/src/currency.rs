//! Stream currency configuration.

use std::collections::HashSet;
use std::sync::Arc;

use anyhow::{Error, Result};
use thiserror::Error;

use crate::api;
pub(crate) use crate::db::models::Balance;
use crate::db::Database;
use crate::injector::Injector;
use crate::stream::StreamExt;
use crate::utils::Duration;

mod builtin;
mod mysql;

/// Balance of a single user.
#[derive(Default)]
pub(crate) struct BalanceOf {
    pub(crate) balance: i64,
    pub(crate) watch_time: i64,
}

impl BalanceOf {
    /// Get the current watch time for the specified balance as a duration.
    pub(crate) fn watch_time(&self) -> Duration {
        if self.watch_time < 0 {
            return Duration::default();
        }

        Duration::seconds(self.watch_time as u64)
    }
}

/// Helper struct to construct a currency.
pub(crate) struct CurrencyBuilder {
    streamer: api::TwitchAndUser,
    pub(crate) mysql_schema: mysql::Schema,
    injector: Injector,
    pub(crate) ty: BackendType,
    pub(crate) enabled: bool,
    pub(crate) command_enabled: bool,
    pub(crate) name: Option<Arc<String>>,
    pub(crate) db: Option<Database>,
    pub(crate) mysql_url: Option<String>,
}

impl CurrencyBuilder {
    /// Construct a new currency builder.
    pub(crate) fn new(
        streamer: api::TwitchAndUser,
        mysql_schema: mysql::Schema,
        injector: Injector,
    ) -> Self {
        Self {
            streamer,
            mysql_schema,
            injector,
            ty: Default::default(),
            enabled: Default::default(),
            command_enabled: Default::default(),
            name: Default::default(),
            db: None,
            mysql_url: None,
        }
    }

    /// Inject the newly built value and return the result.
    pub(crate) async fn build_and_inject(&self) -> Option<Currency> {
        match self.build() {
            Some(currency) => {
                self.injector.update(currency.clone()).await;
                Some(currency)
            }
            None => {
                self.injector.clear::<Currency>().await;
                None
            }
        }
    }

    /// Build a new currency.
    pub(crate) fn build(&self) -> Option<Currency> {
        use self::mysql::Schema;

        if !self.enabled {
            return None;
        }

        let backend = match self.ty {
            BackendType::BuiltIn => {
                let db = self.db.as_ref()?;
                let backend = self::builtin::Backend::new(db.clone());
                Backend::BuiltIn(backend)
            }
            BackendType::Mysql => {
                let channel = String::from("");
                let url = self.mysql_url.clone()?;
                let schema = self.mysql_schema.clone();

                let backend = match self::mysql::Backend::connect(channel, url, schema) {
                    Ok(backend) => backend,
                    Err(e) => {
                        log_error!(e, "Failed to establish connection");
                        return None;
                    }
                };

                Backend::MySql(backend)
            }
            BackendType::Honkos => {
                let channel = String::from("");
                let url = self.mysql_url.clone()?;
                let schema = Schema {
                    table: String::from("honkos"),
                    user_column: String::from("username"),
                    balance_column: String::from("honko_balance"),
                };

                let backend = match self::mysql::Backend::connect(channel, url, schema) {
                    Ok(backend) => backend,
                    Err(e) => {
                        log_error!(e, "Failed to establish connection");
                        return None;
                    }
                };

                Backend::MySql(backend)
            }
        };

        Some(Currency {
            name: Arc::new(self.name.as_ref()?.to_string()),
            command_enabled: self.command_enabled,
            inner: Arc::new(Inner {
                backend,
                streamer: self.streamer.clone(),
            }),
        })
    }
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, Default)]
pub(crate) enum BackendType {
    #[serde(rename = "builtin")]
    #[default]
    BuiltIn,
    #[serde(rename = "mysql")]
    Mysql,
    #[serde(rename = "honkos")]
    Honkos,
}

enum Backend {
    BuiltIn(self::builtin::Backend),
    MySql(self::mysql::Backend),
}

impl Backend {
    /// Add (or subtract) from the balance for a single user.
    pub(crate) async fn balance_transfer(
        &self,
        channel: &str,
        giver: &str,
        taker: &str,
        amount: i64,
        override_balance: bool,
    ) -> Result<(), BalanceTransferError> {
        use self::Backend::*;

        match *self {
            BuiltIn(ref backend) => {
                backend
                    .balance_transfer(channel, giver, taker, amount, override_balance)
                    .await
            }
            MySql(ref backend) => {
                backend
                    .balance_transfer(channel, giver, taker, amount, override_balance)
                    .await
            }
        }
    }

    /// Get balances for all users.
    pub(crate) async fn export_balances(&self) -> Result<Vec<Balance>> {
        use self::Backend::*;

        match *self {
            BuiltIn(ref backend) => backend.export_balances().await,
            MySql(ref backend) => backend.export_balances().await,
        }
    }

    /// Import balances for all users.
    pub(crate) async fn import_balances(&self, balances: Vec<Balance>) -> Result<()> {
        use self::Backend::*;

        match *self {
            BuiltIn(ref backend) => backend.import_balances(balances).await,
            MySql(ref backend) => backend.import_balances(balances).await,
        }
    }

    /// Find user balance.
    pub(crate) async fn balance_of(&self, channel: &str, user: &str) -> Result<Option<BalanceOf>> {
        use self::Backend::*;

        match *self {
            BuiltIn(ref backend) => backend.balance_of(channel, user).await,
            MySql(ref backend) => backend.balance_of(channel, user).await,
        }
    }

    /// Add (or subtract) from the balance for a single user.
    pub(crate) async fn balance_add(&self, channel: &str, user: &str, amount: i64) -> Result<()> {
        use self::Backend::*;

        match *self {
            BuiltIn(ref backend) => backend.balance_add(channel, user, amount).await,
            MySql(ref backend) => backend.balance_add(channel, user, amount).await,
        }
    }

    /// Add balance to users.
    #[tracing::instrument(skip(self, users))]
    pub(crate) async fn balances_increment<I>(
        &self,
        channel: &str,
        users: I,
        amount: i64,
        watch_time: i64,
    ) -> Result<()>
    where
        I: IntoIterator<Item = String> + Send + 'static,
        I::IntoIter: Send,
    {
        use self::Backend::*;

        match *self {
            BuiltIn(ref backend) => {
                backend
                    .balances_increment(channel, users, amount, watch_time)
                    .await
            }
            MySql(ref backend) => backend.balances_increment(channel, users, amount).await,
        }
    }
}

struct Inner {
    backend: Backend,
    streamer: api::TwitchAndUser,
}

/// The currency being used.
#[derive(Clone)]
pub(crate) struct Currency {
    pub(crate) name: Arc<String>,
    pub(crate) command_enabled: bool,
    inner: Arc<Inner>,
}

impl Currency {
    /// Reward all users.
    #[tracing::instrument(skip(self))]
    pub(crate) async fn add_channel_all(
        &self,
        reward: i64,
        watch_time: i64,
    ) -> Result<usize, anyhow::Error> {
        tracing::trace!("Getting chatters");

        let chatters = self
            .inner
            .streamer
            .client
            .chatters(&self.inner.streamer.user.id, &self.inner.streamer.user.id);
        tokio::pin!(chatters);

        let mut users = HashSet::new();

        while let Some(chatter) = chatters.next().await.transpose()? {
            users.insert(chatter.user_login);
        }

        let len = users.len();

        self.inner
            .backend
            .balances_increment(
                self.inner.streamer.user.login.as_str(),
                users,
                reward,
                watch_time,
            )
            .await?;

        Ok(len)
    }

    /// Add (or subtract) from the balance for a single user.
    pub(crate) async fn balance_transfer(
        &self,
        giver: &str,
        taker: &str,
        amount: i64,
        override_balance: bool,
    ) -> Result<(), BalanceTransferError> {
        self.inner
            .backend
            .balance_transfer(
                &self.inner.streamer.user.login,
                giver,
                taker,
                amount,
                override_balance,
            )
            .await
    }

    /// Get balances for all users.
    pub(crate) async fn export_balances(&self) -> Result<Vec<Balance>> {
        self.inner.backend.export_balances().await
    }

    /// Import balances for all users.
    pub(crate) async fn import_balances(&self, balances: Vec<Balance>) -> Result<()> {
        self.inner.backend.import_balances(balances).await
    }

    /// Find user balance.
    pub(crate) async fn balance_of(&self, user: &str) -> Result<Option<BalanceOf>> {
        self.inner
            .backend
            .balance_of(&self.inner.streamer.user.login, user)
            .await
    }

    /// Add (or subtract) from the balance for a single user.
    pub(crate) async fn balance_add(&self, user: &str, amount: i64) -> Result<()> {
        self.inner
            .backend
            .balance_add(&self.inner.streamer.user.login, user, amount)
            .await
    }

    /// Add balance to users.
    pub(crate) async fn balances_increment<I>(
        &self,
        users: I,
        amount: i64,
        watch_time: i64,
    ) -> Result<()>
    where
        I: IntoIterator<Item = String> + Send + 'static,
        I::IntoIter: Send + 'static,
    {
        self.inner
            .backend
            .balances_increment(&self.inner.streamer.user.login, users, amount, watch_time)
            .await
    }
}

#[derive(Debug, Error)]
pub(crate) enum BalanceTransferError {
    #[error("missing balance for transfer")]
    NoBalance,
    #[error("other error: {}", _0)]
    Other(#[source] Error),
}

impl From<tokio::task::JoinError> for BalanceTransferError {
    fn from(error: tokio::task::JoinError) -> Self {
        Self::Other(Error::from(error))
    }
}

impl From<Error> for BalanceTransferError {
    fn from(value: Error) -> Self {
        BalanceTransferError::Other(value)
    }
}

impl From<diesel::result::Error> for BalanceTransferError {
    fn from(value: diesel::result::Error) -> Self {
        BalanceTransferError::Other(value.into())
    }
}

impl From<std::num::TryFromIntError> for BalanceTransferError {
    fn from(value: std::num::TryFromIntError) -> Self {
        BalanceTransferError::Other(value.into())
    }
}

impl From<mysql_async::Error> for BalanceTransferError {
    fn from(value: mysql_async::Error) -> Self {
        BalanceTransferError::Other(value.into())
    }
}
