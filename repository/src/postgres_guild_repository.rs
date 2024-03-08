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

// PostgresUtGuildをUtGuildに変換する

impl From<PostgresUtGuild> for UtGuild {
    fn from(p: PostgresUtGuild) -> Self {
        Self {
            guild_id: p.guild_id.digits(),
            guild_name: p.guild_name,
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

        Ok(guild.into())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::GuildRepository;
    use dotenvy::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    #[tokio::test]
    async fn test_upsert_guild() {
        dotenv().ok();

        let pool = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();
        let repository = PostgresGuildRepository::new(pool);

        let guild = UtGuild {
            // 20桁の数値を格納できるかどうか確認するため
            guild_id: 10101010101010100101,
            guild_name: Some("test_guild".to_string()),
        };

        repository.upsert_guild(guild).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_guild() {
        // 入れたデータと取り出したデータが一致するかどうか確認する
        dotenv().ok();

        let pool = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();
        let repository = PostgresGuildRepository::new(pool);

        let guild = UtGuild {
            guild_id: 10101010101010100101,
            guild_name: Some("test_guild".to_string()),
        };

        repository.upsert_guild(guild.clone()).await.unwrap();

        let fetched_guild = repository.get_guild(guild.guild_id).await.unwrap();
        assert_eq!(fetched_guild, guild);
    }

    #[tokio::test]
    async fn test_delete_guild() {
        dotenv().ok();

        let pool = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();
        let repository = PostgresGuildRepository::new(pool);

        let guild = UtGuild {
            guild_id: 123,
            guild_name: Some("test_guild".to_string()),
        };

        repository.upsert_guild(guild.clone()).await.unwrap();

        repository.delete_guild(guild.guild_id).await.unwrap();
    }
}
