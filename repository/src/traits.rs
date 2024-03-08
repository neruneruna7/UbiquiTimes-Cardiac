use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// FromRowをここでつけておく
// 薄いラッパ(ニュータイプパターン)を使えば，ここでなくて具体的にやってる側で書けるかも？
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtTime {
    pub user_id: u64,
    pub user_name: String,
    pub guild_id: u64,
    pub channel_id: u64,
    pub webhook_url: u64,
}

pub trait TimesRepository {
    type Error;
    fn add_time(&self, time: UtTime) -> Result<(), Self::Error>;
    fn update_time(&self, time: UtTime) -> Result<(), Self::Error>;
    fn get_times(&self, user_id: u64, guild_id: u64) -> Result<Vec<UtTime>, Self::Error>;
    fn delete_time(&self, user_id: u64, guild_id: u64) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtGuild {
    guild_id: u64,
    guild_name: String,
}

pub trait GuildRepository {
    // ここはErrorではなくResultでもいいのだが，Errorに着目するためあえ今回はこの形をとっている
    type Error;
    fn add_guild(&self, guild: UtGuild) -> Result<(), Self::Error>;
    fn update_guild(&self, guild: UtGuild) -> Result<(), Self::Error>;
    fn get_guild(&self, guild_id: u64) -> Result<UtGuild, Self::Error>;
    fn delete_guild(&self, guild_id: u64) -> Result<(), Self::Error>;
}
