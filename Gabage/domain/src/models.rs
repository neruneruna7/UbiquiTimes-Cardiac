#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UtGuild {
    pub guild_id: u64,
    pub guild_name: Option<String>,
}

impl UtGuild {
    pub fn new(guild_id: u64, guild_name: Option<String>) -> Self {
        Self {
            guild_id,
            guild_name,
        }
    }
}

// FromRowをここでつけておく
// 薄いラッパ(ニュータイプパターン)を使えば，ここでなくて具体的にやってる側で書けるかも？
// FromRowをここでつけるのは不要となった
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UtTime {
    pub user_id: u64,
    pub guild_id: u64,
    pub user_name: String,
    pub channel_id: u64,
    pub webhook_url: String,
}

impl UtTime {
    pub fn new(
        user_id: u64,
        guild_id: u64,
        user_name: String,
        channel_id: u64,
        webhook_url: String,
    ) -> Self {
        Self {
            user_id,
            guild_id,
            user_name,
            channel_id,
            webhook_url,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimesMessage {
    pub avater_url: String,
    pub content: String,
}
