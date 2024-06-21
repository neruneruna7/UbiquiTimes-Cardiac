//!Discord用のデータへDBアクセスするときの便利処理などをまとめたモジュール
//!

use share::model::{DiscordCommunity, DiscordNewUser, DiscordTimes, User};
use sqlx::{prelude::FromRow, types::BigDecimal};

#[derive(Debug, Clone, FromRow)]
pub struct PostgresNewUser {
    pub discord_user_id: Option<BigDecimal>,
}

impl From<DiscordNewUser> for PostgresNewUser {
    fn from(user: DiscordNewUser) -> Self {
        PostgresNewUser {
            discord_user_id: Some(BigDecimal::from(user.discord_user_id)),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct PostgresUser {
    pub id: i32,
    pub discord_user_id: Option<BigDecimal>,
    pub slack_user_id: Option<String>,
    pub token: Option<String>,
    pub random_int: Option<i32>,
}

impl From<User> for PostgresUser {
    fn from(user: User) -> Self {
        PostgresUser {
            id: user.id,
            discord_user_id: user.discord_user_id.map(BigDecimal::from),
            slack_user_id: user.slack_user_id,
            token: user.token,
            random_int: user.random_int,
        }
    }
}

impl From<PostgresUser> for User {
    fn from(pg_user: PostgresUser) -> Self {
        User {
            id: pg_user.id,
            discord_user_id: pg_user.discord_user_id.map(|d| {
                d.to_string()
                    .parse::<u64>()
                    .expect("BigDecimal to u64 conversion failed")
            }),
            slack_user_id: pg_user.slack_user_id,
            token: pg_user.token,
            random_int: pg_user.random_int,
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct PostgresGuild {
    pub guild_id: BigDecimal,
    pub guild_name: String,
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
pub struct PostgresTime {
    pub user_id: BigDecimal,
    pub guild_id: BigDecimal,
    pub user_name: String,
    pub channel_id: BigDecimal,
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
