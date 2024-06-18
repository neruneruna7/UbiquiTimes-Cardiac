use domain::models::UtTime;
use domain::repository::TimesRepository;

use thiserror::Error;

use sqlx::{types::BigDecimal, FromRow, PgPool};

use sqlx::Error as SqlxError;

// tracingもロギングも全く理解していないことだらけだが，とりあえず使ってみる
use tracing::{info, instrument};

#[derive(Error, Debug)]
pub enum PostgresTimesRepositoryError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] SqlxError),
}

// postgresではu64を格納できないので，Bigdecimalに変換して格納する

#[derive(Debug, Clone, FromRow)]
struct PostgresUtTime {
    user_id: BigDecimal,
    guild_id: BigDecimal,
    user_name: String,
    channel_id: BigDecimal,
    webhook_url: String,
}

// UtTimeをPostgresUtTimeに変換する

impl From<UtTime> for PostgresUtTime {
    fn from(u: UtTime) -> Self {
        Self {
            user_id: BigDecimal::from(u.user_id),
            guild_id: BigDecimal::from(u.guild_id),
            user_name: u.user_name,
            channel_id: BigDecimal::from(u.channel_id),
            webhook_url: u.webhook_url,
        }
    }
}

// PostgresUtTimeをUtTimeに変換する

impl From<PostgresUtTime> for UtTime {
    fn from(p: PostgresUtTime) -> Self {
        Self {
            user_id: p.user_id.to_string().parse().unwrap(),
            guild_id: p.guild_id.to_string().parse().unwrap(),
            user_name: p.user_name,
            channel_id: p.channel_id.to_string().parse().unwrap(),
            webhook_url: p.webhook_url,
        }
    }
}

pub struct PostgresTimesRepository {
    pool: PgPool,
}

impl PostgresTimesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TimesRepository for PostgresTimesRepository {
    type Error = PostgresTimesRepositoryError;

    #[instrument(skip(self))]
    async fn upsert_and_return_old_time(
        &self,
        time: UtTime,
    ) -> Result<Option<UtTime>, Self::Error> {
        let postgres_time = PostgresUtTime::from(time);
        // 現在の値を取得する
        // ない場合はNoneを返す
        // ある場合はその値を返す
        // トランザクションを開始する
        // トランザクションを実際のコード中でどう扱えばいいのかよくわからない
        let mut tx = self.pool.begin().await?;

        let old_time: Option<PostgresUtTime> = sqlx::query_as(
            r#"
            SELECT user_id, guild_id, user_name, channel_id, webhook_url
            FROM times
            WHERE user_id = $1 AND guild_id = $2
            "#,
        )
        .bind(&postgres_time.user_id)
        .bind(&postgres_time.guild_id)
        .fetch_optional(&mut *tx)
        .await?;

        // 異常終了した場合はロールバックしたいが，どうやるのかよくわからない
        // 明示しなくても自動でロールバックされるのだろうか

        // 衝突した場合は，前の値を取得したあとに新しい値で更新する
        sqlx::query(
            r#"
            INSERT INTO times (user_id, guild_id, user_name, channel_id, webhook_url)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (user_id, guild_id) DO UPDATE
            SET user_name = $3, channel_id = $4, webhook_url = $5

            "#,
        )
        .bind(&postgres_time.user_id)
        .bind(&postgres_time.guild_id)
        .bind(&postgres_time.user_name)
        .bind(&postgres_time.channel_id)
        .bind(&postgres_time.webhook_url)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        info!(
            "time upserted successfully in postgres. user_id: {}, guild_id: {}. rerurned: {:?}",
            postgres_time.user_id, postgres_time.guild_id, postgres_time
        );

        if let Some(old_time) = old_time {
            Ok(Some(old_time.into()))
        } else {
            Ok(None)
        }
    }

    /// user_idと一致するTimeをすべて取得する
    async fn get_times(&self, user_id: u64) -> Result<Vec<UtTime>, Self::Error> {
        let bigdecimal_user_id = BigDecimal::from(user_id);
        let times: Vec<PostgresUtTime> = sqlx::query_as(
            r#"
            SELECT user_id, guild_id, user_name, channel_id, webhook_url
            FROM times
            WHERE user_id = $1
            "#,
        )
        .bind(bigdecimal_user_id)
        .fetch_all(&self.pool)
        .await?;

        info!(
            "times fetched successfully from postgres. user_id: {}",
            user_id
        );

        let times = times.into_iter().map(|t| t.into()).collect();

        Ok(times)
    }

    async fn delete_time(&self, user_id: u64, guild_id: u64) -> Result<(), Self::Error> {
        let bigdecimal_user_id = BigDecimal::from(user_id);
        let bigdecimal_guild_id = BigDecimal::from(guild_id);

        sqlx::query(
            r#"
            DELETE FROM times
            WHERE user_id = $1 AND guild_id = $2
            "#,
        )
        .bind(bigdecimal_user_id)
        .bind(bigdecimal_guild_id)
        .execute(&self.pool)
        .await?;

        info!(
            "time deleted successfully from postgres. user_id: {}, guild_id: {}",
            user_id, guild_id
        );

        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_time(&self, user_id: u64, guild_id: u64) -> Result<UtTime, Self::Error> {
        let bigdecimal_user_id = BigDecimal::from(user_id);
        let bigdecimal_guild_id = BigDecimal::from(guild_id);
        let time: PostgresUtTime = sqlx::query_as(
            r#"
            SELECT user_id, guild_id, user_name, channel_id, webhook_url
            FROM times
            WHERE user_id = $1 AND guild_id = $2
            "#,
        )
        .bind(bigdecimal_user_id)
        .bind(bigdecimal_guild_id)
        .fetch_one(&self.pool)
        .await?;

        info!(
            "time fetched successfully from postgres. user_id: {}, guild_id: {}",
            user_id, guild_id
        );

        Ok(time.into())
    }
}

#[cfg(test)]
mod tests;
