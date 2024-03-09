use message_sender::poise_webhook_message_sender::PoiseWebhookMessageSenderError;
use poise::serenity_prelude::{self as serenity, model::error};

use repository::{
    postgres_guild_repository::PostgresGuildRepositoryError,
    postgres_times_repository::PostgresTimesRepositoryError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UbiquiTimesCardiacError {
    #[error("std error: {0}")]
    StdError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("shuttle runtime error: {0}")]
    ShuttleRuntimeError(#[from] shuttle_runtime::Error),
    #[error("serenity error: {0}")]
    SerenityError(#[from] serenity::Error),
    #[error("guild repository error: {0}")]
    GuildRepositoryError(#[from] PostgresGuildRepositoryError),
    #[error("times repository error: {0}")]
    TimesRepositoryError(#[from] PostgresTimesRepositoryError),
    #[error("guild get error: {0}")]
    GuildGetError(#[from] GuildGetError),
    #[error("user get error: {0}")]
    UserGetError(#[from] UserGetError),
    #[error("poise webhook message sender error: {0}")]
    PoiseWebhookMessageSenderError(#[from] PoiseWebhookMessageSenderError),
}

pub type UbiquiTimesCardiacResult<T> = Result<T, UbiquiTimesCardiacError>;

// Guild情報を取得できないエラー
#[derive(Debug, Clone)]
pub struct GuildGetError;

impl std::fmt::Display for GuildGetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GuildGetError")
    }
}

impl std::error::Error for GuildGetError {
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
pub struct UserGetError;

impl std::fmt::Display for UserGetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "UserGetError")
    }
}

impl std::error::Error for UserGetError {
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
