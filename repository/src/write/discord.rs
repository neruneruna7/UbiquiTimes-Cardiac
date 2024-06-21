use crate::util::discord::*;
use share::model::{DiscordCommunity, DiscordTimes, User};
use sqlx::{prelude::FromRow, types::BigDecimal};
use tracing::info;

pub struct Repository {
    pool: sqlx::PgPool,
}

impl Repository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(skip(self))]
    pub async fn upsert_user(&self, user: User) -> anyhow::Result<()> {
        let user = PostgresUser::from(user);
        let query = "
        INSERT INTO users (id, discord_user_id, slack_user_id, token, random_int)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id) DO UPDATE
        SET discord_user_id = EXCLUDED.discord_user_id,
            slack_user_id = EXCLUDED.slack_user_id,
            token = EXCLUDED.token,
            random_int = EXCLUDED.random_int;
    ";

        sqlx::query(query)
            .bind(user.id)
            .bind(user.discord_user_id)
            .bind(user.slack_user_id)
            .bind(user.token)
            .bind(user.random_int)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn upsert_guild_time(
        &self,
        community: DiscordCommunity,
        times: DiscordTimes,
    ) -> anyhow::Result<()> {
        // ギルド情報を登録
        self.upsert_guilds(community).await?;

        // Times情報を登録
        self.upsert_times(times).await?;

        Ok(())
    }

    async fn upsert_times(&self, times: DiscordTimes) -> Result<(), anyhow::Error> {
        let times = PostgresTime::from(times);
        let _query = sqlx::query(
            r"
            INSERT INTO discord_times (user_id, guild_id, user_name, channel_id) 
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id, guild_id) 
            DO UPDATE SET user_name = $3, channel_id = $4",
        )
        .bind(times.user_id)
        .bind(times.guild_id)
        .bind(times.user_name)
        .bind(times.channel_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn upsert_guilds(&self, community: DiscordCommunity) -> Result<(), anyhow::Error> {
        let guild = PostgresGuild::from(community);
        let _query = sqlx::query(
            r"
            INSERT INTO discord_guilds (guild_id, guild_name) 
            VALUES ($1, $2)
            ON CONFLICT (guild_id) 
            DO UPDATE SET guild_name = $2",
        )
        .bind(guild.guild_id)
        .bind(guild.guild_name)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// user_idと一致するTimeをすべて取得する
    #[tracing::instrument(skip(self))]
    pub async fn read_times(&self, user_id: u64) -> Result<Vec<DiscordTimes>, anyhow::Error> {
        let bigdecimal_user_id = BigDecimal::from(user_id);
        let times: Vec<PostgresTime> = sqlx::query_as(
            r#"
                SELECT user_id, guild_id, user_name, channel_id
                FROM discord_times
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
}

#[cfg(test)]
mod tests {
    use crate::test_util::{generate_random_20_digits, setup_postgres_testcontainer};

    use super::*;

    #[tokio::test]
    async fn test_upsert_user_new_user() -> Result<(), anyhow::Error> {
        // Setup test environment
        let (_container, pool) = setup_postgres_testcontainer().await;
        let repository = Repository::new(pool);

        // Create a new user
        let user = User {
            id: 1,
            discord_user_id: Some(generate_random_20_digits()),
            slack_user_id: Some("test_id".to_string()),
            token: Some("test_token".to_string()),
            random_int: Some(42),
        };

        // Attempt to upsert the new user
        repository.upsert_user(user.clone()).await?;

        // Verify the user was inserted
        let fetched_user: Option<PostgresUser> =
            sqlx::query_as("SELECT * FROM users WHERE id = $1")
                .bind(&user.id)
                .fetch_optional(&repository.pool)
                .await?;

        assert!(fetched_user.is_some());
        let fetched_user = User::from(fetched_user.unwrap());
        assert_eq!(fetched_user.discord_user_id, user.discord_user_id);
        assert_eq!(fetched_user.slack_user_id, user.slack_user_id);
        assert_eq!(fetched_user.token, user.token);
        assert_eq!(fetched_user.random_int, user.random_int);

        Ok(())
    }

    #[tokio::test]
    async fn test_upsert_user_update_existing() -> Result<(), anyhow::Error> {
        // Setup test environment
        let (_container, pool) = setup_postgres_testcontainer().await;
        let repository = Repository::new(pool);

        // Create and insert a user
        let mut user = User {
            id: 1,
            discord_user_id: Some(generate_random_20_digits()),
            slack_user_id: Some("initial_id".to_string()),
            token: Some("initial_token".to_string()),
            random_int: Some(24),
        };

        repository.upsert_user(user.clone()).await?;

        // Update the user's details
        user.discord_user_id = Some(generate_random_20_digits());
        user.slack_user_id = Some("updated_id".to_string());
        user.token = Some("updated_token".to_string());
        user.random_int = Some(48);

        // Attempt to upsert the updated user
        repository.upsert_user(user.clone()).await?;

        // Verify the user was updated
        let fetched_user: Option<PostgresUser> =
            sqlx::query_as("SELECT * FROM users WHERE id = $1")
                .bind(&user.id)
                .fetch_optional(&repository.pool)
                .await?;

        assert!(fetched_user.is_some());
        let fetched_user = User::from(fetched_user.unwrap());
        assert_eq!(fetched_user.discord_user_id, user.discord_user_id);
        assert_eq!(fetched_user.slack_user_id, user.slack_user_id);
        assert_eq!(fetched_user.token, user.token);
        assert_eq!(fetched_user.random_int, user.random_int);

        Ok(())
    }

    #[tokio::test]
    async fn test_upsert() -> Result<(), anyhow::Error> {
        // Create a new database pool for testing
        let (_container, pool) = setup_postgres_testcontainer().await;
        // Create a new instance of the Repository
        let repository = Repository::new(pool);

        // Create test data
        let community = DiscordCommunity {
            guild_id: generate_random_20_digits(),
            guild_name: "Test Guild".to_string(),
        };
        
        // ユーザーテストデータ
        let user = User {
            id: 1,
            discord_user_id: Some(1),
            slack_user_id: Some("test_id".to_string()),
            token: Some("test_token".to_string()),
            random_int: Some(42),
        };

        let times = DiscordTimes {
            user_id: user.discord_user_id.unwrap(),
            guild_id: community.guild_id,
            user_name: "Test User".to_string(),
            channel_id: generate_random_20_digits(),
        };

        repository.upsert_user(user.clone()).await?;

        // Call the upsert method
        repository
            .upsert_guild_time(community.clone(), times.clone())
            .await?;



        let community = PostgresGuild::from(community);
        let times = PostgresTime::from(times);



        // Verify that the guild and times data is inserted or updated correctly
        let guild =
            sqlx::query_as::<_, PostgresGuild>("SELECT * FROM discord_guilds WHERE guild_id = $1")
                .bind(community.guild_id)
                .fetch_one(&repository.pool)
                .await?;
        assert_eq!(guild.guild_name, community.guild_name);

        let time = sqlx::query_as::<_, PostgresTime>(
            "SELECT * FROM discord_times WHERE user_id = $1 AND guild_id = $2",
        )
        .bind(times.user_id)
        .bind(times.guild_id)
        .fetch_one(&repository.pool)
        .await?;
        assert_eq!(time.user_name, times.user_name);
        assert_eq!(time.channel_id, times.channel_id);

        Ok(())
    }

    #[tokio::test]
    async fn test_read_times() -> Result<(), anyhow::Error> {
        // Create a new database pool for testing
        let (_container, pool) = setup_postgres_testcontainer().await;
        // Create a new instance of the Repository
        let repository = Repository::new(pool);
        // Create test data
        let user_id = generate_random_20_digits();
        let user1 = User {
            id: 1,
            discord_user_id: Some(user_id),
            slack_user_id: Some("test_id1".to_string()),
            token: Some("test_token".to_string()),
            random_int: Some(42),
        };

        repository.upsert_user(user1.clone()).await?;



        let guild_id1 = generate_random_20_digits();
        let guild1 = DiscordCommunity {
            guild_id: guild_id1,
            guild_name: "Test Guild".to_string(),
        };

        let guild_id2 = generate_random_20_digits();
        let guild2 = DiscordCommunity {
            guild_id: guild_id2,
            guild_name: "Test Guild 2".to_string(),
        };
        let times = vec![
            (
                guild1,
                DiscordTimes {
                    user_id: user1.discord_user_id.unwrap(),
                    guild_id: guild_id1,
                    user_name: "User One".to_string(),
                    channel_id: generate_random_20_digits(),
                },
                user1.clone(),
            ),
            (
                guild2,
                DiscordTimes {
                    user_id: user1.discord_user_id.unwrap(),
                    guild_id: guild_id2,
                    user_name: "User Two".to_string(),
                    channel_id: generate_random_20_digits(),
                },
                user1,
            ),
        ];

        // Insert test data
        for i in times.iter() {

            repository
                .upsert_guild_time(i.0.clone(), i.1.clone())
                .await?;
        }
        // Call the read_times method
        let mut fetched_times = repository.read_times(user_id).await?;

        let mut times = times
            .into_iter()
            .map(|(_, t, _)| t)
            .collect::<Vec<DiscordTimes>>();
        times.sort_by_key(|x| (x.guild_id, x.user_id));
        fetched_times.sort_by_key(|x| (x.guild_id, x.user_id));
        // Verify that the correct number of times are fetched
        assert_eq!(fetched_times.len(), times.len());
        // Verify that the fetched times match the inserted times
        for (fetched, inserted) in fetched_times.iter().zip(times.iter()) {
            assert_eq!(fetched.user_id, inserted.user_id);
            assert_eq!(fetched.guild_id, inserted.guild_id);
            assert_eq!(fetched.user_name, inserted.user_name);
            assert_eq!(fetched.channel_id, inserted.channel_id);
        }

        Ok(())
    }
}
