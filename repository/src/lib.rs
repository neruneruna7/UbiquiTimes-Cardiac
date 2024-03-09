pub mod postgres_guild_repository;
pub mod postgres_times_repository;
pub mod traits;

pub use traits::{UtGuild, UtTime};

#[cfg(test)]
mod test_utils {
    use rand::Rng;

    // ランダムな20桁の数値を生成する
    // discordの各種idが20桁の数値であるため，それに合わせる
    pub(crate) fn generate_random_20_digits() -> u64 {
        let mut rng = rand::thread_rng();
        let random_20_digits = rng.gen_range(10000000000000000000..=u64::MAX);
        random_20_digits
    }
}
