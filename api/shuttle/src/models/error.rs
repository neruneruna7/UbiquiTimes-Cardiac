use poise::serenity_prelude as serenity;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UbiquiTimesCardiacError {
    #[error("std error: {0}")]
    StdError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("shuttle runtime error: {0}")]
    ShuttleRuntimeError(#[from] shuttle_runtime::Error),
    #[error("serenity error: {0}")]
    SerenityError(#[from] serenity::Error),
}

pub type UbiquiTimesCardiacResult<T> = Result<T, UbiquiTimesCardiacError>;
