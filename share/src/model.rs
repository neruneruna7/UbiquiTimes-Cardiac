#[derive(Debug, Clone)]
pub enum Times {
    Discord(DiscordTimes),
    Slack,
}

#[derive(Debug, Clone)]
pub struct DiscordTimes {
    pub user_id: u64,
    pub guild_id: u64,
    pub user_name: String,
    pub channel_id: u64,
}

impl DiscordTimes {
    pub fn new(user_id: u64, guild_id: u64, user_name: String, channel_id: u64) -> Self {
        Self {
            user_id,
            guild_id,
            user_name,
            channel_id,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Community {
    Discord,
    Slack,
}

#[derive(Debug, Clone)]
pub struct DiscordCommunity {
    pub guild_id: u64,
    pub guild_name: String,
}

impl DiscordCommunity {
    pub fn new(guild_id: u64, guild_name: String) -> Self {
        Self {
            guild_id,
            guild_name,
        }
    }
}