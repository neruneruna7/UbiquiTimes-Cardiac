use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;

use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::{CustomError, SecretStore};
use shuttle_serenity::ShuttleSerenity;
use sqlx::{Executor, PgPool};

use tracing::info;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleSerenity {
    // dbのセットアップ
    // Create the tables if they don't exist yet
    // Ok()
    todo!()
}
