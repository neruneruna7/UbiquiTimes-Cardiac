#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordNewUser {
    pub discord_user_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackNewUser {
    pub slack_user_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub discord_user_id: Option<u64>,
    pub slack_user_id: Option<String>,
    pub token: Option<String>,
    pub random_int: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Times {
    Discord(DiscordTimes),
    Slack,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Community {
    Discord,
    Slack,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PubMessage {
    Discord(DiscordPubMessage),
    Slack,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordPubMessage {
    pub user_id: u64,
    pub channel_id: u64,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackPubMessage {
    pub user_id: String,
    pub channel_id: String,
    pub content: String,
}
