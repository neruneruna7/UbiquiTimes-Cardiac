use serde::{Deserialize, Serialize};
use thiserror::Error;

use sqlx::{types::BigDecimal, Executor, FromRow, PgPool};

use sqlx::Error as SqlxError;

use crate::traits::{GuildRepository, UtGuild};

#[derive(Error, Debug)]
pub enum PostgresGuildRepositoryError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] SqlxError),
}

// postgresではu64を格納できないので，Bigdecimalに変換して格納する

#[derive(Debug, Clone, FromRow)]
struct PostgresUtGuild {
    guild_id: BigDecimal,
    guild_name: String,
}

// UtGuildをPostgresUtGuildに変換する

impl From<UtGuild> for PostgresUtGuild {
    fn from(u: UtGuild) -> Self {
        Self {
            guild_id: BigDecimal::from(u.guild_id),
            guild_name: u.guild_name,
        }
    }
}
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

    async fn add_guild(&self, guild: UtGuild) -> Result<(), Self::Error> {
        let postgres_guild = PostgresUtGuild::from(guild);
        // 正しく動くのかどうか確認するためだけのクエリ
        sqlx::query!(
            r#"
            INSERT INTO guilds (guild_id, guild_name)
            VALUES ($1, $2)
            "#,
            postgres_guild.guild_id,
            postgres_guild.guild_name
        )
        .execute(&self.pool)
        .await
        .map_err(|e| PostgresGuildRepositoryError::from(e))
        .unwrap();

        todo!()
    }

    async fn update_guild(&self, guild: UtGuild) -> Result<(), Self::Error> {
        todo!()
    }

    async fn get_guild(&self, guild_id: u64) -> Result<UtGuild, Self::Error> {
        todo!()
    }

    async fn delete_guild(&self, guild_id: u64) -> Result<(), Self::Error> {
        todo!()
    }
}
