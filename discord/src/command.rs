use anyhow::Context as _;
use share::{model::{DiscordCommunity, DiscordTimes}, util::ubiquitimes_user_name};
use tracing::info;

use crate::{repository::Repository, Context, Error};


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
