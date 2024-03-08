use serde::{Deserialize, Serialize};
use thiserror::Error;

use sqlx::{Executor, FromRow, PgPool};

use crate::traits::{GuildRepository, UtGuild};

#[derive(Error, Debug)]
pub enum PostgresGuildRepositoryError {
    //
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
struct UtGuildWrappeer(UtGuild);

pub struct PostgresGuildRepository {
    pool: PgPool,
}

impl PostgresGuildRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GuildRepository for PostgresGuildRepository {
    type Error = PostgresGuildRepository;

    fn add_guild(&self, guild: UtGuild) -> Result<(), Self::Error> {
        todo!()
    }

    fn update_guild(&self, guild: UtGuild) -> Result<(), Self::Error> {
        todo!()
    }

    fn get_guild(&self, guild_id: u64) -> Result<UtGuild, Self::Error> {
        todo!()
    }

    fn delete_guild(&self, guild_id: u64) -> Result<(), Self::Error> {
        todo!()
    }
}
