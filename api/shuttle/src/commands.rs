// - ut-c_guild_init
// 	- ギルドの情報を登録する
// 	- サーバー管理者が実行するコマンド
// 	- ただ，実行するユーザーに依存しない
// 	- ギルドのidと，ギルドの名前を取得してDBに保存する
// - ut-c_times_set
// 	- 実行するユーザーに依存
// 	- 実行するチャンネルに依存
// 	- 実行したチャンネルをそのユーザーのTimesとしてDBに保存する
// 	- ２度目以降は更新と同じ
// - ut-c_times_delete
// 	- 実行するユーザーに依存
// 	- 実行したユーザーのTimes情報をDBから削除する
// - ut-c_times_release
// 	- 実行するユーザーに依存
// 	- 実行するチャンネルに依存
// 	- 実行するギルドに依存
// 	- 保存されたTimes情報のchannel_idと一致しない場合，チャンネル不一致として弾く
// 	- 実行したギルド以外の，Timesが登録されているすべてのギルドへ同じ内容を送信する

use crate::models::error::{GuildGetError, UserGetError};
use crate::models::{Context, Data, UbiquiTimesCardiacResult as Result};
use crate::webhook_creator::create_webhook_url;
use domain::models::{TimesMessage, UtTime};
use domain::{
    message_sender::TimesMessageSender,
    models::UtGuild,
    repository::{GuildRepository, TimesRepository},
};
use poise::serenity_prelude::{ChannelId, CreateWebhook, Webhook};
use tracing::info;

/// Responds with "world!"
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<()> {
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
) -> Result<()> {
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

#[poise::command(prefix_command, track_edits, aliases("UtInit"), slash_command)]
#[tracing::instrument(skip(ctx))]
/// bot導入後，最初に実行してください
///
/// ギルドの情報を登録します
/// 現在は誰が実行しても同じです
/// guild_idとguild_nameをbot側に保存します
pub async fn ut_c_guild_init(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().ok_or(GuildGetError)?.get();
    let guild_name = ctx.guild().ok_or(GuildGetError)?.name.clone();

    let guilds_repository = ctx.data().guild_repository.clone();
    let guild = UtGuild::new(guild_id, Some(guild_name.clone()));
    guilds_repository.upsert_guild(guild).await?;

    ctx.say(format!(
        "guild_id: {}, guild_name: {}",
        guild_id, guild_name
    ))
    .await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, aliases("UtTimesSet"), slash_command)]
#[tracing::instrument(skip(ctx))]
/// 実行したチャンネルをあなたのTimesとして登録します
///
/// ２度目以降の実行は情報を更新します
pub async fn ut_c_times_set(
    ctx: Context<'_>,
    #[description = "このギルドで使用する名前"] user_name: String,
) -> Result<()> {
    let user_id = ctx.author().id.get();
    let guild_id = ctx.guild_id().ok_or(GuildGetError)?.get();
    let channel_id = ctx.channel_id();
    let channel_id_u64 = channel_id.get();

    let webhook_url = create_webhook_url(ctx).await?;

    let times_repository = ctx.data().times_repository.clone();

    let time = UtTime::new(
        user_id,
        guild_id,
        user_name.clone(),
        channel_id.get(),
        webhook_url.clone(),
    );
    let old_time = times_repository.upsert_time(time).await?;
    info!(
        "new times set complete. guild_id: {}, user_id: {}, channel_id: {}",
        guild_id, user_id, channel_id_u64
    );

    // 古いwebhookを削除
    let old_webhook_url = old_time.webhook_url;
    let webhook = Webhook::from_url(ctx, &old_webhook_url).await?;
    webhook.delete(ctx).await?;

    info!("Webhook deleted: {}", old_webhook_url);

    ctx.say("Success! I learned that this channel is your Times!")
        .await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, aliases("UtTimesDelete"), slash_command)]
#[tracing::instrument(skip(ctx))]
/// あなたのTimes情報を削除します
pub async fn ut_c_times_delete(ctx: Context<'_>) -> Result<()> {
    let user_id = ctx.author().id.get();
    let guild_id = ctx.guild_id().ok_or(GuildGetError)?.get();

    let times_repository = ctx.data().times_repository.clone();
    times_repository.delete_time(user_id, guild_id).await?;

    ctx.say("Success! I forgot your Times!").await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, aliases("UT"), slash_command)]
#[tracing::instrument(skip(ctx))]
/// 書き込んだ内容を拡散します
///
/// 書き込んだ内容を登録されたギルドのすべてのチャンネルに送信します
pub async fn ut_c_times_release(
    ctx: Context<'_>,
    #[description = "送信する内容"] content: String,
) -> Result<()> {
    let times_message = TimesMessage {
        avater_url: ctx.author().avatar_url().unwrap_or_default(),
        content: content.clone(),
    };

    let user_id = ctx.author().id.get();

    let times_repository = ctx.data().times_repository.clone();
    let times = times_repository.get_times(user_id).await?;

    let message_sender = ctx.data().times_message_sender.clone();
    message_sender.send_all(times_message, times).await?;

    info!("times release complete. user_id: {}", user_id);
    Ok(())
}
