use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtGuild {
    guild_id: u64,
    guild_name: String,
}

pub trait GuildRepository {
    fn add_guild(&self, guild: UtGuild) -> Result<(), Error>;
    fn update_guild(&self, guild: UtGuild) -> Result<(), Error>;
    fn get_guild(&self, guild_id: u64) -> Result<UtGuild, Error>;
    fn delete_guild(&self, guild_id: u64) -> Result<(), Error>;
}
