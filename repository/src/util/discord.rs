//!Discord用のデータへDBアクセスするときの便利処理などをまとめたモジュール
//!

use share::model::{DiscordCommunity, DiscordTimes};
use sqlx::{prelude::FromRow, types::BigDecimal};

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
