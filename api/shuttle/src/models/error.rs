use poise::serenity_prelude as serenity;

use repository::postgres_guild_repository::PostgresGuildRepositoryError;
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
}

pub type UbiquiTimesCardiacResult<T> = Result<T, UbiquiTimesCardiacError>;
