use std::sync::Arc;

use message_sender::poise_webhook_message_sender::PoiseWebhookMessageSender;
use repository::postgres_guild_repository::PostgresGuildRepository;
use repository::postgres_times_repository::PostgresTimesRepository;
use sqlx::PgPool;

// User data, which is stored and accessible in all command invocations
// #[derive(Debug)]
pub(crate) struct Data {
    pub pool: PgPool,
    pub guild_repository: Arc<PostgresGuildRepository>,
    pub times_repository: Arc<PostgresTimesRepository>,
    pub times_message_sender: Arc<PoiseWebhookMessageSender>,
}
