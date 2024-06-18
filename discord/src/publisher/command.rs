use anyhow::Context as _;
use share::{
    model::{DiscordCommunity, DiscordTimes},
    util::ubiquitimes_user_name,
};
use tracing::info;

use crate::repository::Repository;

use super::{Context, Error};

#[poise::command(prefix_command, track_edits, aliases("UtTimesSet"), slash_command)]
#[tracing::instrument(skip(ctx))]
/// 実行したチャンネルをあなたのTimesとして登録します
///
/// ２度目以降の実行は情報を更新します
pub async fn ut_c_times_set(
    ctx: Context<'_>,
    #[description = "このギルドで使用する名前"] user_name: String,
) -> Result<(), Error> {
    let user_id = ctx.author().id.get();
    let guild_id = ctx.guild_id().unwrap().get();
    let channel_id = ctx.channel_id();
    let channel_id_u64 = channel_id.get();

    let guild = DiscordCommunity {
        guild_id,
        guild_name: ctx.guild().context("guild data not found")?.name.clone(),
    };

    // Ubiquitimesから拡散だとわかるように，ユーザー名にプレフィックスを付加する
    let user_name = ubiquitimes_user_name(user_name);
    // Timesを作成
    let time = DiscordTimes::new(user_id, guild_id, user_name.clone(), channel_id.get());

    // DBに保存
    let repository = Repository::new(ctx.data().pool.clone());
    repository.upsert(guild, time).await?;

    info!(
        "new times set complete. guild_id: {}, user_id: {}, channel_id: {}",
        guild_id, user_id, channel_id_u64
    );

    let reply_mesage = format!(
        "Success! Hello {}, I learned that this channel is your Times!",
        user_name
    );

    ctx.say(reply_mesage).await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, aliases("UT"), slash_command)]
#[tracing::instrument(skip(ctx))]
/// 代わりに~UTプレフィックスコマンドを使用してください
///
/// 書き込んだ内容を，他のギルドのあなたのTimesへ送信します
/// ~UTプレフィックスコマンドを使用してください
pub async fn ut_c_times_release(
    ctx: Context<'_>,
    // 複数行のメッセージを受け取るためにVec<String>を使用
    // 使用されないが，引数がないとエラーになるため，ダミーの引数を追加
    #[description = "message"] _content: Vec<String>,
) -> Result<(), Error> {
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

    // 最初の~UTを削除
    let content = remove_ut_prefix(&prefix_ctx.msg.content);
    info!("content: {:?}", content);

    // Times情報を取得
    // let user_id = ctx.author().id.get();

    // let times_repository = ctx.data().times_repository.clone();
    // let times = times_repository.get_times(user_id).await?;

    // Timesから，発信元のguild_idを持ったTimeを削除
    let guild_id = ctx.guild_id().context("guild not found")?.get();
    // let times = times
    //     .into_iter()
    //     .filter(|t| t.guild_id != guild_id)
    //     .collect();

    // channelで送信

    // info!("times release complete. user_id: {}", user_id);
    Ok(())
}

fn remove_ut_prefix(msg_content: &str) -> String {
    msg_content.trim_start_matches("~UT\n").to_string()
}

#[poise::command(prefix_command, hide_in_help)]
#[tracing::instrument(skip(ctx))]
///  スラッシュコマンドの変更を即座に反映するためのコマンド
///
/// 主にデバッグ用
pub async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, global).await?;
    Ok(())
}
