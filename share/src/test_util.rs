use rand::Rng;
use sqlx::{Executor as _, PgPool};
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::{self, Postgres};
use testcontainers_modules::testcontainers::runners::AsyncRunner;

// ランダムな20桁の数値を生成する
// discordの各種idが20桁の数値であるため，それに合わせる
#[allow(dead_code)]
pub(crate) fn generate_random_20_digits() -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(10000000000000000000..=u64::MAX)
}

/// コンテナの生存期間を，呼び出し元にゆだねるために，コンテナの変数を返す
pub async fn setup_postgres_testcontainer() -> (ContainerAsync<Postgres>, PgPool) {
    let container = postgres::Postgres::default().start().await.unwrap();
    let host_port = container.get_host_port_ipv4(5432).await.unwrap();
    let connection_string =
        &format!("postgres://postgres:postgres@127.0.0.1:{host_port}/postgres",);
    let pool = PgPool::connect(connection_string).await.unwrap();
    // スキーマをセットアップする
    pool.execute(include_str!("../../core/db/schema.sql"))
        .await
        .unwrap();

    (container, pool)
}
