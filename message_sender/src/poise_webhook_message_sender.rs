use poise::serenity_prelude::{ExecuteWebhook, Http, Webhook};
use thiserror::Error;
use tracing::info;

use crate::traits::{TimesMessage, TimesMessageSender};

#[derive(Debug, Error)]
pub enum PoiseWebhookMessageSenderError {
    #[error("Webhook error: {0}")]
    WebhookError(#[from] poise::serenity_prelude::Error),
}

#[derive(Debug)]
pub struct PoiseWebhookMessageSender;

impl PoiseWebhookMessageSender {
    pub fn new() -> Self {
        Self
    }
}

impl TimesMessageSender for PoiseWebhookMessageSender {
    type Error = PoiseWebhookMessageSenderError;

    #[tracing::instrument(skip(self))]
    async fn send_all(
        &self,
        message: TimesMessage,
        times: Vec<repository::UtTime>,
    ) -> Result<(), Self::Error> {
        // Webhookを送るだけなら，トークンとやらはなしでもいいらしい
        let http = Http::new("");

        for time in times.into_iter() {
            let webhook = Webhook::from_url(&http, &time.webhook_url).await?;
            let builder = ExecuteWebhook::new()
                .content(message.content.clone())
                .username(time.user_name)
                .avatar_url(message.avater_url.clone());
            webhook.execute(&http, false, builder).await?;
        }

        info!("send_all complete");
        Ok(())
    }
}
