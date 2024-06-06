use super::*;
use domain::models::UtGuild;

#[cfg(test)]
use crate::test_utils::generate_random_20_digits;
use crate::test_utils::setup_postgres_testcontainer;

#[tokio::test]
async fn test_upsert_guild() {
    let (_container, pool) = setup_postgres_testcontainer().await;

    let repository = PostgresGuildRepository::new(pool);

    let guild_id = generate_random_20_digits();
    let guild = UtGuild {
        // 20桁の数値を格納できるかどうか確認するため
        guild_id,
        guild_name: Some("test_guild".to_string()),
    };

    repository.upsert_guild(guild).await.unwrap();
}

#[tokio::test]
async fn test_get_guild() {
    // 入れたデータと取り出したデータが一致するかどうか確認する
    let (_container, pool) = setup_postgres_testcontainer().await;

    let repository = PostgresGuildRepository::new(pool);

    let guild_id = generate_random_20_digits();

    let guild = UtGuild {
        guild_id,
        guild_name: Some("test_guild".to_string()),
    };

    repository.upsert_guild(guild.clone()).await.unwrap();

    let fetched_guild = repository.get_guild(guild.guild_id).await.unwrap();
    assert_eq!(fetched_guild, guild);
}

#[tokio::test]
async fn test_delete_guild() {
    let (_container, pool) = setup_postgres_testcontainer().await;

    let repository = PostgresGuildRepository::new(pool);

    let guild_id = generate_random_20_digits();

    let guild = UtGuild {
        guild_id,
        guild_name: Some("test_guild".to_string()),
    };

    repository.upsert_guild(guild.clone()).await.unwrap();

    repository.delete_guild(guild.guild_id).await.unwrap();
}
