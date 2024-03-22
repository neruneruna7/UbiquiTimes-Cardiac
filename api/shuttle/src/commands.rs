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

use crate::models::error::GuildGetError;
use crate::models::{Context, UbiquiTimesCardiacResult as Result};
use domain::models::UtTime;
use domain::ubiquitimes_user_name::ubiquitimes_user_name;
use domain::ubiquitimes_webhook_name::webhook_name;
use domain::{
    message_sender::TimesMessageSender,
    models::UtGuild,
    repository::{GuildRepository, TimesRepository},
};

use poise::serenity_prelude::{CreateWebhook, Webhook};
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

    let guild = UtGuild::new(guild_id, Some(guild_name.clone()));

    let guilds_repository = ctx.data().guild_repository.clone();
    guilds_repository.upsert_guild(guild).await?;

    let reply_mesage = format!(
        "Success! Welcome {},  I learned this guild! {}",
        guild_name, guild_id
    );

    ctx.say(reply_mesage).await?;
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

    // Ubiquitimesから拡散だとわかるように，ユーザー名にプレフィックスを付加する
    let user_name = ubiquitimes_user_name(user_name);
    let webhook_name = webhook_name(user_id);

    // コマンド実行のたびに新しいwebhookを作成し，古いwebhookを削除する
    // 処理の複雑さを減らすためと，予期せぬWebhookの無効化で，Webhook再作成の条件にあわないのに無効化されて動作しなくなることを防ぐため
    let builder = CreateWebhook::new(webhook_name);
    let webhook = ctx.channel_id().create_webhook(&ctx, builder).await?;
    let webhook_url = webhook.url()?;

    // Timeデータを作成
    let time = UtTime::new(
        user_id,
        guild_id,
        user_name.clone(),
        channel_id.get(),
        webhook_url.clone(),
    );

    // Timeデータを保存または更新
    let times_repository = ctx.data().times_repository.clone();
    let old_time = times_repository.upsert_time(time).await?;

    // 古いwebhookを削除
    delete_old_webhook(&ctx, old_time).await?;

    info!(
        "new times set complete. guild_id: {}, user_id: {}, channel_id: {}, webhook_url: {}",
        guild_id, user_id, channel_id_u64, webhook_url
    );

    let reply_mesage = format!(
        "Success! Hello {}, I learned that this channel is your Times!",
        user_name
    );

    ctx.say(reply_mesage).await?;
    Ok(())
}

async fn delete_old_webhook(ctx: &Context<'_>, old_time: Option<UtTime>) -> Result<()> {
    if let Some(old_time) = old_time {
        let old_webhook_url = old_time.webhook_url;
        let webhook = Webhook::from_url(ctx, &old_webhook_url).await?;
        webhook.delete(ctx).await?;

        info!("Webhook deleted: {}", old_webhook_url);
    }
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
/// 代わりに~UTプレフィックスコマンドを使用してください
///
/// 書き込んだ内容を，他のギルドのあなたのTimesへ送信します
/// ~UTプレフィックスコマンドを使用してください
/// スラッシュコマンドで使用した場合，アプリケーションの応答がないと返ってきますが，
/// 無視してください
pub async fn ut_c_times_release(
    ctx: Context<'_>,
    // 複数行のメッセージを受け取るためにVec<String>を使用
    #[description = "message"] content: Vec<String>,
) -> Result<()> {
    info!("message: {:?}", content);
    let prefix_ctx = match &ctx {
        poise::Context::Application(_) => {
            // スラッシュコマンド非対応なのに，なぜスラッシュコマンドとして呼ぶことも許可しているのか？
            // コマンドを探すときに，スラッシュを打ったら候補に出てくる方が探しやすいから
            // その後．プレフィックスコマンドに誘導する方が良いと考えた
            info!("slash command is not supported. please use the ~UT prefix command.");
            let _ = ctx.say("Please use the ~UT prefix command").await;
            return Ok(());
        }
        poise::Context::Prefix(prefix_ctx) => prefix_ctx,
    };
    info!("prefix command");

    // ベクタを改行でつなげて，元の文字列に戻す
    let content = content.join("\n");

    let user_id = ctx.author().id.get();

    let times_repository = ctx.data().times_repository.clone();
    let times = times_repository.get_times(user_id).await?;

    // Timesから，発信元のguild_idを持ったTimeを削除
    let guild_id = ctx.guild_id().ok_or(GuildGetError)?.get();
    let times = times
        .into_iter()
        .filter(|t| t.guild_id != guild_id)
        .collect();

    let message_sender = ctx.data().times_message_sender.clone();
    message_sender
        .send_message_all_times(prefix_ctx.msg, content, times)
        .await?;

    info!("times release complete. user_id: {}", user_id);
    Ok(())
}

#[poise::command(prefix_command, hide_in_help)]
#[tracing::instrument(skip(ctx))]
///  スラッシュコマンドの変更を即座に反映するためのコマンド
///
/// 主にデバッグ用
pub async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<()> {
    poise::builtins::register_application_commands(ctx, global).await?;
    Ok(())
}

/// テスト用のコマンド
#[poise::command(prefix_command, track_edits, aliases("test"), slash_command)]
#[tracing::instrument(skip(ctx))]
pub async fn ut_c_test(
    ctx: Context<'_>,
    #[description = "message"] content: Vec<String>,
) -> Result<()> {
    let content = content.join("\n");
    info!("message: {:?}", content);
    ctx.say(content).await?;
    Ok(())
}
