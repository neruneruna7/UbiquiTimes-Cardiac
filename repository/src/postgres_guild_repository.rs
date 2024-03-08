use serde::{Deserialize, Serialize};
use thiserror::Error;

use sqlx::{types::BigDecimal, Executor, FromRow, PgPool};

use sqlx::Error as SqlxError;

// tracingもロギングも全く理解していないことだらけだが，とりあえず使ってみる
use tracing::{info, instrument};

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
    guild_name: Option<String>,
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
    type Error = PostgresGuildRepositoryError;

    #[instrument(skip(self))]
    async fn upsert_guild(&self, guild: UtGuild) -> Result<(), Self::Error> {
        let postgres_guild = PostgresUtGuild::from(guild);
        // 正しく動くのかどうか確認するためだけのクエリ
        sqlx::query!(
            r#"
            INSERT INTO guilds (guild_id, guild_name)
            VALUES ($1, $2)
            ON CONFLICT (guild_id) DO UPDATE
            SET guild_name = $2
            "#,
            postgres_guild.guild_id,
            postgres_guild.guild_name
        )
        .execute(&self.pool)
        .await?;

        info!(
            "guild upserted successfully in postgres. guild_id: {}",
            postgres_guild.guild_id
        );
        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_guild(&self, guild_id: u64) -> Result<UtGuild, Self::Error> {
        let bigdecimal_guild_id = BigDecimal::from(guild_id);
        let guild = sqlx::query_as!(
            PostgresUtGuild,
            r#"
            SELECT guild_id, guild_name
            FROM guilds
            WHERE guild_id = $1
            "#,
            bigdecimal_guild_id
        )
        .fetch_one(&self.pool)
        .await?;

        info!(
            "guild fetched successfully from postgres. guild_id: {}, guild_name: {}",
            guild.guild_id,
            guild.guild_name.as_deref().unwrap_or("None")
        );

        Ok(UtGuild {
            guild_id: guild.guild_id.to_string().parse().unwrap(),
            guild_name: guild.guild_name,
        })
    }

    #[instrument(skip(self))]
    async fn delete_guild(&self, guild_id: u64) -> Result<(), Self::Error> {
        let bigdecimal_guild_id = BigDecimal::from(guild_id);
        sqlx::query!(
            r#"
            DELETE FROM guilds
            WHERE guild_id = $1
            "#,
            bigdecimal_guild_id
        )
        .execute(&self.pool)
        .await?;

        info!(
            "guild deleted successfully from postgres. guild_id: {}",
            guild_id
        );
        Ok(())
    }
}
