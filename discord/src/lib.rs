use std::{sync::Arc, time::Duration};

use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use tracing::info;
use anyhow::{Context as _, Result};

pub struct DiscordArg {
    pub discord_bot_token: String,
}

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[tracing::instrument(skip(arg))]
pub async fn start_discord_bot(arg: DiscordArg) -> Result<()>{
    let discord_token = arg.discord_bot_token;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                hello(),
                help(),
            ],
            // ここでprefixを設定する
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    Duration::from_secs(3600),
                ))),
                additional_prefixes: vec![
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
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                })
            })
        })
        .build();

    let _client = ClientBuilder::new(
        discord_token,
        GatewayIntents::non_privileged()
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_WEBHOOKS,
    )
    .framework(framework)
    .await?
    .start()
    .await?;

    Ok(())
}

/// Responds with "world!"
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    // 動作確認用のコマンド
    ctx.say("world!").await?;
    Ok(())
}

/// ヘルプを表示します
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}