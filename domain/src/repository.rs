use crate::models::{UtGuild, UtTime};

pub trait TimesRepository {
    type Error;
    async fn upsert_time(&self, time: UtTime) -> Result<UtTime, Self::Error>;
    async fn get_time(&self, user_id: u64, guild_id: u64) -> Result<UtTime, Self::Error>;
    async fn get_times(&self, user_id: u64) -> Result<Vec<UtTime>, Self::Error>;
    async fn delete_time(&self, user_id: u64, guild_id: u64) -> Result<(), Self::Error>;
}

pub trait GuildRepository {
    // ここはErrorではなくResultでもいいのだが，Errorに着目するためあえ今回はこの形をとっている
    type Error;
    async fn upsert_guild(&self, guild: UtGuild) -> Result<(), Self::Error>;
    async fn get_guild(&self, guild_id: u64) -> Result<UtGuild, Self::Error>;
    async fn delete_guild(&self, guild_id: u64) -> Result<(), Self::Error>;
}
