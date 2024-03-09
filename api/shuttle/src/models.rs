pub mod data;
pub mod error;

pub(crate) use data::Data;
pub(crate) use error::UbiquiTimesCardiacError;
pub(crate) use error::UbiquiTimesCardiacResult;

pub(crate) type Context<'a> = poise::Context<'a, Data, UbiquiTimesCardiacError>;
