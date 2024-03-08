use crate::models::error::UbiquiTimesCardiacResult as Result;
use poise::serenity_prelude::{model::webhook, CreateWebhook};

use crate::models::Context;

/// Weebhook名はUT-c_{user_id}とする
pub const WEBHOOK_NAME_PREFIX: &str = "UT-c_";

pub async fn create_webhook_url(ctx: Context<'_>) -> Result<String> {
    let user_id = ctx.author().id.get();
    let webhook_name = format!("{}{}", WEBHOOK_NAME_PREFIX, user_id);

    // チャンネルに存在するwebhookを確認
    // 指定フォーマットの名前のwebhookが存在する場合，そのwebhookのurlを返す
    // 存在しない場合，新規作成してそのurlを返す
    let channel_id = ctx.channel_id();
    let webhooks = ctx.http().get_channel_webhooks(channel_id).await?;

    // Check if a webhook with the specified format exists
    if let Some(webhook) = webhooks
        .iter()
        .find(|w| w.name == Some(webhook_name.clone()))
    {
        let webhook_url = webhook.url()?;
        return Ok(webhook_url);
    }

    // If no webhook with the specified format exists, create a new one
    let builder = CreateWebhook::new(webhook_name);
    let webhook = ctx.channel_id().create_webhook(&ctx, builder).await?;
    let webhook_url = webhook.url()?;

    Ok(webhook_url)
}
