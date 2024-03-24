use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use message_sender::poise_webhook_message_sender::PoiseWebhookMessageSender;

use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::{CustomError, SecretStore};
use shuttle_serenity::ShuttleSerenity;
use sqlx::{Executor, PgPool};

mod commands;
mod models;

use models::Data;

use domain::tracing::{self, info};
use repository::postgres_guild_repository::PostgresGuildRepository;
use repository::postgres_times_repository::PostgresTimesRepository;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleSerenity {
    // dbのセットアップ
    // Create the tables if they don't exist yet

    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    use commands::{
        hello, help, register, ut_c_guild_init, ut_c_test, ut_c_times_delete, ut_c_times_release,
        ut_c_times_set,
    };
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                hello(),
                help(),
                ut_c_guild_init(),
                ut_c_times_set(),
                ut_c_times_delete(),
                ut_c_times_release(),
                register(),
                ut_c_test(),
            ],
            // ここでprefixを設定する
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    Duration::from_secs(3600),
                ))),
                additional_prefixes: vec![
                    poise::Prefix::Literal("hey bot"),
                    poise::Prefix::Literal("hey bot,"),
                ],
                ..Default::default()
            },
            // This code is run before every command
            // このコードはすべてのコマンドの前に実行されます
            pre_command: |ctx| {
                Box::pin(async move {
                    info!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            // This code is run after a command if it was successful (returned Ok)
            // このコードは、コマンドが成功した場合 (Ok が返された場合)、コマンドの後に実行されます。
            post_command: |ctx| {
                Box::pin(async move {
                    info!("Executed command {}!", ctx.command().qualified_name);
                })
            },

            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            // poolをcloneしてもよいのだろうか？
            // 不明である
            let guild_repository = Arc::new(PostgresGuildRepository::new(pool.clone()));
            let times_repository = Arc::new(PostgresTimesRepository::new(pool));
            let times_message_sender = Arc::new(PoiseWebhookMessageSender::new());
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    guild_repository,
                    times_repository,
                    times_message_sender,
                })
            })
        })
        .build();

    let client = ClientBuilder::new(
        discord_token,
        GatewayIntents::non_privileged()
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_WEBHOOKS,
    )
    .framework(framework)
    .await
    .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
