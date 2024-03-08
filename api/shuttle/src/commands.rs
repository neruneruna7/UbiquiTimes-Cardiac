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

use crate::models::{Context, Data, UbiquiTimesCardiacResult as Result};
use repository::traits::GuildRepository;
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
    let guild_id = ctx.guild_id().unwrap().get();
    let guild_name = ctx.guild().unwrap().name.clone();

    let guilds_repository = ctx.data().guild_repository.clone();
    let guild = repository::UtGuild::new(guild_id, Some(guild_name.clone()));
    guilds_repository.upsert_guild(guild).await?;

    ctx.say(format!(
        "guild_id: {}, guild_name: {}",
        guild_id, guild_name
    ))
    .await?;
    Ok(())
}
