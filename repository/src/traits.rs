use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// FromRowをここでつけておく
// 薄いラッパ(ニュータイプパターン)を使えば，ここでなくて具体的にやってる側で書けるかも？
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UtTime {
    pub user_id: u64,
    pub guild_id: u64,
    pub user_name: String,
    pub channel_id: u64,
    pub webhook_url: String,
}

pub trait TimesRepository {
    type Error;
    async fn upsert_time(&self, time: UtTime) -> Result<(), Self::Error>;
    async fn get_time(&self, user_id: u64, guild_id: u64) -> Result<UtTime, Self::Error>;
    async fn get_times(&self, user_id: u64) -> Result<Vec<UtTime>, Self::Error>;
    async fn delete_time(&self, user_id: u64, guild_id: u64) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UtGuild {
    pub guild_id: u64,
    pub guild_name: Option<String>,
}

pub trait GuildRepository {
    // ここはErrorではなくResultでもいいのだが，Errorに着目するためあえ今回はこの形をとっている
    type Error;
    async fn upsert_guild(&self, guild: UtGuild) -> Result<(), Self::Error>;
    async fn get_guild(&self, guild_id: u64) -> Result<UtGuild, Self::Error>;
    async fn delete_guild(&self, guild_id: u64) -> Result<(), Self::Error>;
}
