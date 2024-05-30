use message_sender::poise_webhook_message_sender::PoiseWebhookMessageSenderError;
use poise::serenity_prelude::{self as serenity};

use repository::{
    postgres_guild_repository::PostgresGuildRepositoryError,
    postgres_times_repository::PostgresTimesRepositoryError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UbiquiTimesCardiacError {
    #[error("std error: {0}")]
    Std(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("shuttle runtime error: {0}")]
    ShuttleRuntime(#[from] shuttle_runtime::Error),
    #[error("serenity error: {0}")]
    Serenity(#[from] serenity::Error),
    #[error("guild repository error: {0}")]
    GuildRepository(#[from] PostgresGuildRepositoryError),
    #[error("times repository error: {0}")]
    TimesRepository(#[from] PostgresTimesRepositoryError),
    #[error("guild get error: {0}")]
    GuildNotFound(#[from] GuildNotFound),
    #[error("user get error: {0}")]
    UserNotFound(#[from] UserNotFound),
    #[error("poise webhook message sender error: {0}")]
    PoiseWebhookMessageSender(#[from] PoiseWebhookMessageSenderError),
}

pub type UbiquiTimesCardiacResult<T> = Result<T, UbiquiTimesCardiacError>;

// Guild情報を取得できないエラー
#[derive(Debug, Clone)]
pub struct GuildNotFound;

impl std::fmt::Display for GuildNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GuildNotFound")
    }
}

impl std::error::Error for GuildNotFound {
    // Errorトレイトが必要だからってつけてみたけど，全然知らないやつだこれ
    // あとで調べて実装する．今回はあとまわし
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[derive(Debug, Clone)]
pub struct UserNotFound;

impl std::fmt::Display for UserNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "UserNotFound")
    }
}

impl std::error::Error for UserNotFound {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}
