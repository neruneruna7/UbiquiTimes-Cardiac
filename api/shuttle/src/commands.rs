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

/// Responds with "world!"
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<()> {
    // 動作確認用のコマンド
    ctx.say("world!").await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, aliases("UtInit"), slash_command)]
#[tracing::instrument(skip(ctx))]
pub async fn ut_c_guild_init(ctx: Context<'_>) -> Result<()> {
    todo!()
}
