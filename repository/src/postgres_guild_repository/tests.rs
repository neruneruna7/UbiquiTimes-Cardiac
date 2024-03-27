use super::*;
use domain::models::UtGuild;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[cfg(test)]
use crate::test_utils::generate_random_20_digits;

#[tokio::test]
async fn test_upsert_guild() {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .unwrap();

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
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .unwrap();
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
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .unwrap();
    let repository = PostgresGuildRepository::new(pool);

    let guild_id = generate_random_20_digits();

    let guild = UtGuild {
        guild_id,
        guild_name: Some("test_guild".to_string()),
    };

    repository.upsert_guild(guild.clone()).await.unwrap();

    repository.delete_guild(guild.guild_id).await.unwrap();
}
