use share::model::{DiscordNewUser, User};
use sqlx::types::BigDecimal;

use super::{PostgresNewUser, PostgresUser, Repository};

impl Repository {
    /// ```rust,ignore
    /// use share::model::DiscordNewUser;
    /// 
    /// let user = DiscordNewUser {
    ///     discord_user_id: 12345678901234567890,
    /// };
    /// let rows_affected = repository.insert_user(user).await?;
    /// 
    /// assert_eq!(rows_affected, 1);
    /// ```
    #[tracing::instrument(skip(self))]
    pub async fn insert_user(&self, user: DiscordNewUser) -> anyhow::Result<u64> {
        let user = PostgresNewUser::from(user);
        let query = "
            INSERT INTO users (discord_user_id)
            VALUES ($1);
        ";

        let r = sqlx::query(query)
            .bind(user.discord_user_id)
            .execute(&self.pool)
            .await?;

        Ok(r.rows_affected())
    }

    pub async fn read_user(&self, discord_user_id: u64) -> anyhow::Result<Option<User>> {
        let discord_user_id = Some(BigDecimal::from(discord_user_id));
        let user: Option<PostgresUser> =
            sqlx::query_as("SELECT * FROM users WHERE discord_user_id = $1")
                .bind(discord_user_id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(user.map(User::from))
    }

    /// 返り値は更新された行数
    /// rows_affected
    #[tracing::instrument(skip(self))]
    pub async fn update_user(&self, user: User) -> anyhow::Result<u64> {
        let user = PostgresUser::from(user);
        let query = "
            UPDATE users
            SET discord_user_id = $1, slack_user_id = $2, token = $3, random_int = $4
            WHERE id = $5;
        ";

        let r = sqlx::query(query)
            .bind(user.discord_user_id)
            .bind(user.slack_user_id)
            .bind(user.token)
            .bind(user.random_int)
            .bind(user.id)
            .execute(&self.pool)
            .await?;

        Ok(r.rows_affected())
    }

    /// 返り値はrows_affected
    #[tracing::instrument(skip(self))]
    pub async fn delete_user(&self, discord_user_id: u64) -> anyhow::Result<u64> {
        let discord_user_id = Some(BigDecimal::from(discord_user_id));
        let query = "
            DELETE FROM users
            WHERE discord_user_id = $1;
        ";

        let r = sqlx::query(query)
            .bind(discord_user_id)
            .execute(&self.pool)
            .await?;

        Ok(r.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use share::model::{DiscordNewUser, User};

    use crate::{
        test_util::{generate_random_20_digits, setup_postgres_testcontainer},
        write::discord::{PostgresNewUser, PostgresUser, Repository},
    };

    #[tokio::test]
    async fn test_insert_user() -> Result<(), anyhow::Error> {
        // Setup test environment
        let (_container, pool) = setup_postgres_testcontainer().await;
        let repository = Repository::new(pool);
        // Create a new Discord user
        let discord_user = DiscordNewUser {
            discord_user_id: generate_random_20_digits(),
        };
        let user = PostgresNewUser::from(discord_user.clone());
        // Attempt to insert the new Discord user
        repository.insert_user(discord_user).await?;
        // Verify the Discord user was inserted
        let fetched_user: Option<PostgresUser> =
            sqlx::query_as("SELECT * FROM users WHERE discord_user_id = $1")
                .bind(user.discord_user_id.clone())
                .fetch_optional(&repository.pool)
                .await?;

        assert!(fetched_user.is_some());
        let fetched_user = fetched_user.unwrap();
        assert_eq!(fetched_user.discord_user_id, user.discord_user_id);
        Ok(())
    }

    #[tokio::test]
    async fn test_read_user_nonexistent() -> Result<(), anyhow::Error> {
        // Setup test environment
        let (_container, pool) = setup_postgres_testcontainer().await;
        let repository = Repository::new(pool);
        // Attempt to read a user that does not exist
        let discord_user_id = generate_random_20_digits();
        let result = repository.read_user(discord_user_id).await?;
        // Verify that no user is returned
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_read_user_existing() -> Result<(), anyhow::Error> {
        // Setup test environment
        let (_container, pool) = setup_postgres_testcontainer().await;
        let repository = Repository::new(pool);
        // Create and insert a new Discord user
        let discord_user = DiscordNewUser {
            discord_user_id: generate_random_20_digits(),
        };
        let rows_affected = repository.insert_user(discord_user.clone()).await?;
        assert_eq!(rows_affected, 1);

        // Attempt to read the inserted user
        let result = repository.read_user(discord_user.discord_user_id).await?;
        // Verify that the user is returned and matches the inserted user
        assert!(result.is_some());
        let fetched_user = result.unwrap();
        assert_eq!(
            fetched_user.discord_user_id,
            Some(discord_user.discord_user_id)
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_update_user() -> Result<(), anyhow::Error> {
        // Setup test environment
        let (_container, pool) = setup_postgres_testcontainer().await;
        let repository = Repository::new(pool);
        // Create and insert a new Discord user
        let discord_user = DiscordNewUser {
            discord_user_id: generate_random_20_digits(),
        };
        let rows_affected = repository.insert_user(discord_user.clone()).await?;
        assert_eq!(rows_affected, 1);

        // Read the inserted user
        let user = repository.read_user(discord_user.discord_user_id).await?.unwrap();
        // Update the user
        let updated_user = User {
            id: user.id,
            discord_user_id: Some(generate_random_20_digits()),
            slack_user_id: Some("test_slack_id".to_string()),
            token: Some("token".to_string()),
            random_int: Some(123),
        };
        let rows_affected = repository.update_user(updated_user.clone()).await?;
        assert_eq!(rows_affected, 1);

        // Read the updated user
        let fetched_user = repository.read_user(updated_user.discord_user_id.unwrap()).await?.unwrap();
        // Verify that the updated user matches the expected values
        assert_eq!(fetched_user, updated_user);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_user() -> Result<(), anyhow::Error> {
        // Setup test environment
        let (_container, pool) = setup_postgres_testcontainer().await;
        let repository = Repository::new(pool);
        // Create and insert a new Discord user
        let discord_user = DiscordNewUser {
            discord_user_id: generate_random_20_digits(),
        };
        let rows_affected = repository.insert_user(discord_user.clone()).await?;
        assert_eq!(rows_affected, 1);

        // Attempt to delete the inserted user
        let rows_affected = repository.delete_user(discord_user.discord_user_id).await?;
        assert_eq!(rows_affected, 1);

        // Verify that the user was deleted
        let result = repository.read_user(discord_user.discord_user_id).await?;
        assert!(result.is_none());
        Ok(())
    }

}
