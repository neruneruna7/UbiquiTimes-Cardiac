pub mod postgres_guild_repository;
pub mod postgres_times_repository;

#[cfg(test)]
mod test_utils {
    use rand::Rng;

    // ランダムな20桁の数値を生成する
    // discordの各種idが20桁の数値であるため，それに合わせる
    #[allow(dead_code)]
    pub(crate) fn generate_random_20_digits() -> u64 {
        let mut rng = rand::thread_rng();

        rng.gen_range(10000000000000000000..=u64::MAX)
    }
}
