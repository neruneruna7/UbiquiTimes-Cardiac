use share::model::{DiscordCommunity, DiscordTimes, Times};
use sqlx::{prelude::FromRow, types::BigDecimal};

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
        let query = sqlx::query(
            "INSERT INTO times (user_id, guild_id, user_name, channel_id) VALUES ($1, $2, $3, $4) \
            ON CONFLICT (user_id, guild_id) DO UPDATE SET user_name = $3, channel_id = $4",
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
        let query = sqlx::query(
            "INSERT INTO guilds (guild_id, guild_name) VALUES ($1, $2) \
            ON CONFLICT (guild_id) DO UPDATE SET guild_name = $2",
        )
        .bind(guild.guild_id)
        .bind(guild.guild_name)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
