// 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
use super::*;
use crate::{
    postgres_guild_repository::PostgresGuildRepository, test_utils::setup_postgres_testcontainer,
};
use domain::{models::UtGuild, repository::GuildRepository};
use sqlx::PgPool;

use crate::test_utils::generate_random_20_digits;

async fn setup_guilds_from_times(pool: &PgPool, times: Vec<UtTime>) {
    let guild_repository = PostgresGuildRepository::new(pool.clone());

    for time in times {
        let guild_name = "guild_name".to_string();
        let guild = UtGuild {
            guild_id: time.guild_id,
            guild_name: Some(guild_name),
        };
        guild_repository.upsert_guild(guild).await.unwrap();
    }
}

#[tokio::test]
/// upsert_and_return_old_timeを１度実行し，その際に成功するかどうかを確認する
async fn test_upsert_and_return_old_time() {
    let (_container, pool) = setup_postgres_testcontainer().await;

    let user_id = generate_random_20_digits();
    let guild_id = generate_random_20_digits();
    let channel_id = generate_random_20_digits();

    // 20桁の数値を格納できるかどうか確認するため
    let time = UtTime {
        user_id,
        guild_id,
        user_name: "user_name".to_string(),
        channel_id,
        webhook_url: "webhook_url".to_string(),
    };

    let times = vec![time.clone()];

    // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
    // そのための処理
    setup_guilds_from_times(&pool, times).await;

    let repository = PostgresTimesRepository::new(pool);

    repository
        .upsert_and_return_old_time(time.clone())
        .await
        .unwrap();
}

#[tokio::test]
/// upsert_and_return_old_timeとget_timesを実行し，入れた値と取り出した値が一致するかどうかを確認する
async fn test_get_time() {
    let (_container, pool) = setup_postgres_testcontainer().await;

    let user_id = generate_random_20_digits();
    let guild_id = generate_random_20_digits();
    let channel_id = generate_random_20_digits();

    // 20桁の数値を格納できるかどうか確認するため
    let time = UtTime {
        user_id,
        guild_id,
        user_name: "user_name".to_string(),
        channel_id,
        webhook_url: "webhook_url".to_string(),
    };

    let times = vec![time.clone()];

    // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
    // そのための処理
    setup_guilds_from_times(&pool, times).await;

    let repository = PostgresTimesRepository::new(pool);

    repository
        .upsert_and_return_old_time(time.clone())
        .await
        .unwrap();

    let times = repository.get_times(time.user_id).await.unwrap();
    assert_eq!(times, vec![time]);
}

#[tokio::test]
/// 複数のtimeを入れた場合，正しく取り出せるかどうかを確認する
async fn test_gets_times() {
    let (_container, pool) = setup_postgres_testcontainer().await;

    let user_id = generate_random_20_digits();
    let guild_id_1 = generate_random_20_digits();
    let channel_id_1 = generate_random_20_digits();

    // 20桁の数値を格納できるかどうか確認するため
    let time_1 = UtTime {
        user_id,
        guild_id: guild_id_1,
        user_name: "user_name".to_string(),
        channel_id: channel_id_1,
        webhook_url: "webhook_url".to_string(),
    };

    let guild_id_2 = generate_random_20_digits();
    let channel_id_2 = generate_random_20_digits();

    let time_2 = UtTime {
        user_id,
        guild_id: guild_id_2,
        user_name: "user_name_2".to_string(),
        channel_id: channel_id_2,
        webhook_url: "webhook_url_2".to_string(),
    };

    let times = vec![time_1.clone(), time_2.clone()];

    // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
    // そのための処理
    setup_guilds_from_times(&pool, times).await;

    let repository = PostgresTimesRepository::new(pool);

    repository
        .upsert_and_return_old_time(time_1.clone())
        .await
        .unwrap();
    repository
        .upsert_and_return_old_time(time_2.clone())
        .await
        .unwrap();

    let times = repository.get_times(user_id).await.unwrap();

    // ２つのベクタを順序に依存せずに比較するためにソートする
    let mut times = times;
    times.sort_by(|a, b| a.guild_id.cmp(&b.guild_id));

    let mut expected_times = vec![time_1, time_2];
    expected_times.sort_by(|a, b| a.guild_id.cmp(&b.guild_id));

    assert_eq!(times, expected_times);
}

#[tokio::test]
/// upsert_and_return_old_timeを２度実行した場合，正しく更新されるかどうかを確認する
async fn test_upsert_and_return_old_time_twice() {
    let (_container, pool) = setup_postgres_testcontainer().await;

    let user_id = generate_random_20_digits();
    let guild_id = generate_random_20_digits();
    let channel_id = generate_random_20_digits();

    // 20桁の数値を格納できるかどうか確認するため
    let time_1 = UtTime {
        user_id,
        guild_id,
        user_name: "user_name".to_string(),
        channel_id,
        webhook_url: "webhook_url".to_string(),
    };

    let times = vec![time_1.clone()];

    // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
    // そのための処理
    setup_guilds_from_times(&pool, times).await;

    let repository = PostgresTimesRepository::new(pool);

    repository
        .upsert_and_return_old_time(time_1.clone())
        .await
        .unwrap();

    // 取り出した値とtime_1が一致するかどうかを確認する
    let times = repository.get_time(user_id, guild_id).await.unwrap();

    assert_eq!(times, time_1);

    let time_2 = UtTime {
        user_id,
        guild_id,
        user_name: "user_name_2".to_string(),
        channel_id,
        webhook_url: "webhook_url_2".to_string(),
    };
    repository
        .upsert_and_return_old_time(time_2.clone())
        .await
        .unwrap();

    // 取り出した値とtime_2が一致するかどうかを確認する
    let times = repository.get_time(user_id, guild_id).await.unwrap();

    assert_eq!(times, time_2);
}

#[tokio::test]
/// upsert_and_return_old_timeとdelete_timeを実行し，入れた値を削除できるかどうかを確認する
async fn test_delete_time() {
    let (_container, pool) = setup_postgres_testcontainer().await;

    let user_id = generate_random_20_digits();
    let guild_id = generate_random_20_digits();
    let channel_id = generate_random_20_digits();

    // 20桁の数値を格納できるかどうか確認するため
    let time = UtTime {
        user_id,
        guild_id,
        user_name: "user_name".to_string(),
        channel_id,
        webhook_url: "webhook_url".to_string(),
    };

    let times = vec![time.clone()];

    // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
    // そのための処理
    setup_guilds_from_times(&pool, times).await;

    let repository = PostgresTimesRepository::new(pool);

    repository
        .upsert_and_return_old_time(time.clone())
        .await
        .unwrap();
    repository
        .delete_time(time.user_id, time.guild_id)
        .await
        .unwrap();
    let times = repository.get_time(time.user_id, time.guild_id).await;
    assert!(times.is_err());
}

// upsertで衝突した際に戻ってくる値が正しいかどうかを確認する
#[tokio::test]
async fn test_upsert_conflict() {
    let (_container, pool) = setup_postgres_testcontainer().await;

    let user_id = generate_random_20_digits();
    let guild_id = generate_random_20_digits();
    let channel_id = generate_random_20_digits();

    // 20桁の数値を格納できるかどうか確認するため
    let time_1 = UtTime {
        user_id,
        guild_id,
        user_name: "user_name".to_string(),
        channel_id,
        webhook_url: "webhook_url".to_string(),
    };

    let times = vec![time_1.clone()];

    // 外部キー制約の都合，guildsテーブルにもデータを入れる必要がある
    // そのための処理
    setup_guilds_from_times(&pool, times).await;

    let repository = PostgresTimesRepository::new(pool);

    repository
        .upsert_and_return_old_time(time_1.clone())
        .await
        .unwrap();
    let time_2 = UtTime {
        user_id,
        guild_id,
        user_name: "user_name_2".to_string(),
        channel_id,
        webhook_url: "webhook_url_2".to_string(),
    };

    let returned_time = repository
        .upsert_and_return_old_time(time_2.clone())
        .await
        .unwrap();
    assert_eq!(returned_time, Some(time_1));
}
