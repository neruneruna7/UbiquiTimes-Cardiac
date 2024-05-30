use crate::models::{UtGuild, UtTime};

pub trait TimesRepository {
    type Error;
    fn upsert_and_return_old_time(
        &self,
        time: UtTime,
    ) -> impl std::future::Future<Output = Result<Option<UtTime>, Self::Error>> + Send;
    fn get_time(
        &self,
        user_id: u64,
        guild_id: u64,
    ) -> impl std::future::Future<Output = Result<UtTime, Self::Error>> + Send;
    fn get_times(
        &self,
        user_id: u64,
    ) -> impl std::future::Future<Output = Result<Vec<UtTime>, Self::Error>> + Send;
    fn delete_time(
        &self,
        user_id: u64,
        guild_id: u64,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
}

pub trait GuildRepository {
    // ここはErrorではなくResultでもいいのだが，Errorに着目するためあえ今回はこの形をとっている
    type Error;
    fn upsert_guild(
        &self,
        guild: UtGuild,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
    fn get_guild(
        &self,
        guild_id: u64,
    ) -> impl std::future::Future<Output = Result<UtGuild, Self::Error>> + Send;
    fn delete_guild(
        &self,
        guild_id: u64,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
}
