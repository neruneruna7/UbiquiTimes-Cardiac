
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

#[derive(Debug, Clone)]
pub enum Community {
    Discord,
    Slack,
}

#[derive(Debug, Clone)]
pub struct DiscordCommunity {
    pub guild_id: u64,
    pub channel_id: u64,
}