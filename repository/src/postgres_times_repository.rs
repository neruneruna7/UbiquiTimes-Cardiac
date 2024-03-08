use serde::{Deserialize, Serialize};
use thiserror::Error;

use sqlx::{types::BigDecimal, Executor, FromRow, PgPool};

use sqlx::Error as SqlxError;

// tracingもロギングも全く理解していないことだらけだが，とりあえず使ってみる
use tracing::{info, instrument};

use crate::traits::{TimesRepository, UtTime};

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
    async fn upsert_time(&self, time: UtTime) -> Result<UtTime, Self::Error> {
        let postgres_time = PostgresUtTime::from(time);

        // 衝突した場合は，前の値を取得したあとに新しい値で更新する
        let postgres_time = sqlx::query_as!(
            PostgresUtTime,
            r#"
            INSERT INTO times (user_id, guild_id, user_name, channel_id, webhook_url)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (user_id, guild_id) DO UPDATE
            SET user_name = $3, channel_id = $4, webhook_url = $5
            RETURNING user_id, guild_id, user_name, channel_id, webhook_url
            "#,
            postgres_time.user_id,
            postgres_time.guild_id,
            postgres_time.user_name,
            postgres_time.channel_id,
            postgres_time.webhook_url
        )
        .fetch_one(&self.pool)
        .await?;

        info!(
            "time upserted successfully in postgres. user_id: {}, guild_id: {}. rerurned: {:?}",
            postgres_time.user_id, postgres_time.guild_id, postgres_time
        );

        Ok(postgres_time.into())
    }

    /// user_idと一致するTimeをすべて取得する
    async fn get_times(&self, user_id: u64) -> Result<Vec<UtTime>, Self::Error> {
        let bigdecimal_user_id = BigDecimal::from(user_id);
        let times = sqlx::query_as!(
            PostgresUtTime,
            r#"
            SELECT user_id, guild_id, user_name, channel_id, webhook_url
            FROM times
            WHERE user_id = $1
            "#,
            bigdecimal_user_id,
        )
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

        sqlx::query!(
            r#"
            DELETE FROM times
            WHERE user_id = $1 AND guild_id = $2
            "#,
            bigdecimal_user_id,
            bigdecimal_guild_id
        )
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
        let time = sqlx::query_as!(
            PostgresUtTime,
            r#"
            SELECT user_id, guild_id, user_name, channel_id, webhook_url
            FROM times
            WHERE user_id = $1 AND guild_id = $2
            "#,
            bigdecimal_user_id,
            bigdecimal_guild_id
        )
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
mod tests {
    // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
    use super::*;
    use crate::{
        postgres_guild_repository::PostgresGuildRepository,
        traits::{GuildRepository as _, UtGuild},
    };
    use core::time;
    use dotenvy::dotenv;
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use std::env;

    #[cfg(test)]
    use crate::test_utils::generate_random_20_digits;

    async fn setup_guilds_from_times(pool: &PgPool, times: Vec<UtTime>) {
        let guild_repository = PostgresGuildRepository::new(pool.clone());

        for time in times {
            let guild_name = "guild_name".to_string();
            let guild = UtGuild {
                guild_id: time.guild_id,
                guild_name: Some(guild_name),
            };
            guild_repository.upsert_guild(guild).await.unwrap();
        }
    }

    #[tokio::test]
    /// upsert_timeを１度実行し，その際に成功するかどうかを確認する
    async fn test_upsert_time() {
        dotenv().ok();

        let pool = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();

        let user_id = generate_random_20_digits();
        let guild_id = generate_random_20_digits();
        let channel_id = generate_random_20_digits();

        // 20桁の数値を格納できるかどうか確認するため
        let time = UtTime {
            user_id,
            guild_id,
            user_name: "user_name".to_string(),
            channel_id,
            webhook_url: "webhook_url".to_string(),
        };

        let times = vec![time.clone()];

        // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
        // そのための処理
        setup_guilds_from_times(&pool, times).await;

        let repository = PostgresTimesRepository::new(pool);

        repository.upsert_time(time.clone()).await.unwrap();
    }

    #[tokio::test]
    /// upsert_timeとget_timesを実行し，入れた値と取り出した値が一致するかどうかを確認する
    async fn test_get_time() {
        dotenv().ok();

        let pool = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();

        let user_id = generate_random_20_digits();
        let guild_id = generate_random_20_digits();
        let channel_id = generate_random_20_digits();

        // 20桁の数値を格納できるかどうか確認するため
        let time = UtTime {
            user_id,
            guild_id,
            user_name: "user_name".to_string(),
            channel_id,
            webhook_url: "webhook_url".to_string(),
        };

        let times = vec![time.clone()];

        // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
        // そのための処理
        setup_guilds_from_times(&pool, times).await;

        let repository = PostgresTimesRepository::new(pool);

        repository.upsert_time(time.clone()).await.unwrap();

        let times = repository.get_times(time.user_id).await.unwrap();
        assert_eq!(times, vec![time]);
    }

    #[tokio::test]
    /// 複数のtimeを入れた場合，正しく取り出せるかどうかを確認する
    async fn test_gets_times() {
        dotenv().ok();

        let pool = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();

        let user_id = generate_random_20_digits();
        let guild_id_1 = generate_random_20_digits();
        let channel_id_1 = generate_random_20_digits();

        // 20桁の数値を格納できるかどうか確認するため
        let time_1 = UtTime {
            user_id,
            guild_id: guild_id_1,
            user_name: "user_name".to_string(),
            channel_id: channel_id_1,
            webhook_url: "webhook_url".to_string(),
        };

        let guild_id_2 = generate_random_20_digits();
        let channel_id_2 = generate_random_20_digits();

        let time_2 = UtTime {
            user_id,
            guild_id: guild_id_2,
            user_name: "user_name_2".to_string(),
            channel_id: channel_id_2,
            webhook_url: "webhook_url_2".to_string(),
        };

        let times = vec![time_1.clone(), time_2.clone()];

        // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
        // そのための処理
        setup_guilds_from_times(&pool, times).await;

        let repository = PostgresTimesRepository::new(pool);

        repository.upsert_time(time_1.clone()).await.unwrap();
        repository.upsert_time(time_2.clone()).await.unwrap();

        let times = repository.get_times(user_id).await.unwrap();

        // ２つのベクタを順序に依存せずに比較するためにソートする
        let mut times = times;
        times.sort_by(|a, b| a.guild_id.cmp(&b.guild_id));

        let mut expected_times = vec![time_1, time_2];
        expected_times.sort_by(|a, b| a.guild_id.cmp(&b.guild_id));

        assert_eq!(times, expected_times);
    }

    #[tokio::test]
    /// upsert_timeを２度実行した場合，正しく更新されるかどうかを確認する
    async fn test_upsert_time_twice() {
        dotenv().ok();

        let pool = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();

        let user_id = generate_random_20_digits();
        let guild_id = generate_random_20_digits();
        let channel_id = generate_random_20_digits();

        // 20桁の数値を格納できるかどうか確認するため
        let time_1 = UtTime {
            user_id,
            guild_id,
            user_name: "user_name".to_string(),
            channel_id,
            webhook_url: "webhook_url".to_string(),
        };

        let times = vec![time_1.clone()];

        // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
        // そのための処理
        setup_guilds_from_times(&pool, times).await;

        let repository = PostgresTimesRepository::new(pool);

        repository.upsert_time(time_1.clone()).await.unwrap();

        // 取り出した値とtime_1が一致するかどうかを確認する
        let times = repository.get_time(user_id, guild_id).await.unwrap();

        assert_eq!(times, time_1);

        let time_2 = UtTime {
            user_id,
            guild_id,
            user_name: "user_name_2".to_string(),
            channel_id,
            webhook_url: "webhook_url_2".to_string(),
        };
        repository.upsert_time(time_2.clone()).await.unwrap();

        // 取り出した値とtime_2が一致するかどうかを確認する
        let times = repository.get_time(user_id, guild_id).await.unwrap();

        assert_eq!(times, time_2);
    }

    #[tokio::test]
    /// upsert_timeとdelete_timeを実行し，入れた値を削除できるかどうかを確認する
    async fn test_delete_time() {
        dotenv().ok();

        let pool = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();

        let user_id = generate_random_20_digits();
        let guild_id = generate_random_20_digits();
        let channel_id = generate_random_20_digits();

        // 20桁の数値を格納できるかどうか確認するため
        let time = UtTime {
            user_id,
            guild_id,
            user_name: "user_name".to_string(),
            channel_id,
            webhook_url: "webhook_url".to_string(),
        };

        let times = vec![time.clone()];

        // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
        // そのための処理
        setup_guilds_from_times(&pool, times).await;

        let repository = PostgresTimesRepository::new(pool);

        repository.upsert_time(time.clone()).await.unwrap();
        repository
            .delete_time(time.user_id, time.guild_id)
            .await
            .unwrap();
        let times = repository.get_time(time.user_id, time.guild_id).await;
        assert!(times.is_err());
    }

    // upsertで衝突した際に戻ってくる値が正しいかどうかを確認する
    #[tokio::test]
    async fn test_upsert_conflict() {
        dotenv().ok();

        let pool = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();

        let user_id = generate_random_20_digits();
        let guild_id = generate_random_20_digits();
        let channel_id = generate_random_20_digits();

        // 20桁の数値を格納できるかどうか確認するため
        let time = UtTime {
            user_id,
            guild_id,
            user_name: "user_name".to_string(),
            channel_id,
            webhook_url: "webhook_url".to_string(),
        };

        let times = vec![time.clone()];

        // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
        // そのための処理
        setup_guilds_from_times(&pool, times).await;

        let repository = PostgresTimesRepository::new(pool);

        repository.upsert_time(time.clone()).await.unwrap();
        let time_2 = UtTime {
            user_id,
            guild_id,
            user_name: "user_name_2".to_string(),
            channel_id,
            webhook_url: "webhook_url_2".to_string(),
        };
        let time = repository.upsert_time(time_2.clone()).await.unwrap();
        assert_eq!(time, time_2);
    }
}
