use share::model::{DiscordCommunity, DiscordTimes, Times};
use sqlx::{prelude::FromRow, types::BigDecimal};
use tracing::info;

#[derive(Debug, Clone, FromRow)]
struct PostgresGuild {
    guild_id: BigDecimal,
    guild_name: String,
}

// DiscordCommunityをPostgresGuildに変換する

impl From<DiscordCommunity> for PostgresGuild {
    fn from(u: DiscordCommunity) -> Self {
        Self {
            guild_id: BigDecimal::from(u.guild_id),
            guild_name: u.guild_name,
        }
    }
}

// PostgresDiscordCommunityをDiscordCommunityに変換する

impl From<PostgresGuild> for DiscordCommunity {
    fn from(p: PostgresGuild) -> Self {
        Self {
            guild_id: p.guild_id.to_string().parse().unwrap(),
            guild_name: p.guild_name,
        }
    }
}

// postgresではu64を格納できないので，Bigdecimalに変換して格納する

#[derive(Debug, Clone, FromRow)]
struct PostgresTime {
    user_id: BigDecimal,
    guild_id: BigDecimal,
    user_name: String,
    channel_id: BigDecimal,
}

// UtTimeをPostgresUtTimeに変換する

impl From<DiscordTimes> for PostgresTime {
    fn from(u: DiscordTimes) -> Self {
        Self {
            user_id: BigDecimal::from(u.user_id),
            guild_id: BigDecimal::from(u.guild_id),
            user_name: u.user_name,
            channel_id: BigDecimal::from(u.channel_id),
        }
    }
}

// PostgresUtTimeをUtTimeに変換する

impl From<PostgresTime> for DiscordTimes {
    fn from(p: PostgresTime) -> Self {
        Self {
            user_id: p.user_id.to_string().parse().unwrap(),
            guild_id: p.guild_id.to_string().parse().unwrap(),
            user_name: p.user_name,
            channel_id: p.channel_id.to_string().parse().unwrap(),
        }
    }
}

pub(crate) struct Repository {
    pool: sqlx::PgPool,
}

impl Repository {
    pub(crate) fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(skip(self))]
    pub async fn upsert(
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
    use share::test_util::{generate_random_20_digits, setup_postgres_testcontainer};

    use super::*;

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

        let times = DiscordTimes {
            user_id: generate_random_20_digits(),
            guild_id: community.guild_id,
            user_name: "Test User".to_string(),
            channel_id: generate_random_20_digits(),
        };

        // Call the upsert method
        repository.upsert(community.clone(), times.clone()).await?;

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
        let mut times = vec![
            (
                guild1,
                DiscordTimes {
                    user_id,
                    guild_id: guild_id1,
                    user_name: "User One".to_string(),
                    channel_id: generate_random_20_digits(),
                },
            ),
            (
                guild2,
                DiscordTimes {
                    user_id,
                    guild_id: guild_id2,
                    user_name: "User Two".to_string(),
                    channel_id: generate_random_20_digits(),
                },
            ),
        ];

        // Insert test data
        for i in times.iter() {
            repository.upsert(i.0.clone(), i.1.clone()).await?;
        }
        // Call the read_times method
        let mut fetched_times = repository.read_times(user_id).await?;
        
        let mut times = times.into_iter().map(|(_, t)| t).collect::<Vec<DiscordTimes>>();
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
