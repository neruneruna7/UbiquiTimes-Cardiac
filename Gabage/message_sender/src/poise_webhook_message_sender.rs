use domain::{message_sender::TimesMessageSender, models::UtTime};
use poise::serenity_prelude::{ExecuteWebhook, Http, Message, Webhook};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum PoiseWebhookMessageSenderError {
    #[error("Webhook error: {0}")]
    WebhookError(#[from] poise::serenity_prelude::Error),
}

#[derive(Debug)]
pub struct PoiseWebhookMessageSender;

impl Default for PoiseWebhookMessageSender {
    fn default() -> Self {
        Self::new()
    }
}

impl PoiseWebhookMessageSender {
    pub fn new() -> Self {
        Self
    }
}

impl TimesMessageSender for PoiseWebhookMessageSender {
    type Error = PoiseWebhookMessageSenderError;
    type Message = Message;

    #[tracing::instrument(skip(self, message, text, times))]
    async fn send_all(
        &self,
        message: &Self::Message,
        text: String,
        times: Vec<UtTime>,
    ) -> Result<(), Self::Error> {
        // Webhookを送るだけなら，トークンとやらはなしでもいいらしい
        let http = Http::new("");
        let avater_url = message.author.avatar_url().unwrap_or_default();

        // ファイルの拡散には，URLを使って，URLを本文に付加する形で対応する
        let files = message.attachments.clone();
        for f in files.iter() {
            info!("file url: {:?}, proxy url: {:?}", f.url, f.proxy_url);
        }

        let files_name_and_url = files
            .into_iter()
            .map(|f| {
                format!(
                    r#"

`content_type: {:?}`
`size: {}`
[{}]({})
"#,
                    f.content_type, f.size, f.filename, f.url
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let text = format!("{}\n{}", text, files_name_and_url);

        for time in times.into_iter() {
            info!(
                "will send guild_id {}, webhook_url {:?}",
                time.guild_id, &time.webhook_url
            );
            let webhook = Webhook::from_url(&http, &time.webhook_url).await?;
            let builder = ExecuteWebhook::new()
                .content(&text)
                .username(time.user_name)
                .avatar_url(&avater_url);
            webhook.execute(&http, false, builder).await?;
        }

        info!("send_all complete");
        Ok(())
    }
}
