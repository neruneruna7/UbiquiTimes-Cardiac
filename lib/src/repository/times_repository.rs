use anyhow::Error;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtTime {
    pub user_id: u64,
    pub user_name: String,
    pub guild_id: u64,
    pub channel_id: u64,
    pub webhook_url: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtTimes {
    pub user_id: u64,
    pub time_vec: Vec<UtTime>,
}

pub trait TimesRepository {
    fn add_time(&self, time: UtTime) -> Result<(), Error>;
    fn update_time(&self, time: UtTime) -> Result<(), Error>;
    fn get_times(&self, user_id: u64, guild_id: u64) -> Result<UtTimes, Error>;
    fn delete_time(&self, user_id: u64, guild_id: u64) -> Result<(), Error>;
}
