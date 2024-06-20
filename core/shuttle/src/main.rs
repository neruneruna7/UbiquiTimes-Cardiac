use anyhow::Context as _;
use discord::publisher::{start_discord_bot, DiscordArg};
use shuttle_runtime::{CustomError, SecretStore};
use slack::start_slack_bot;
use sqlx::{Executor as _, PgPool};
use tokio::sync::{broadcast, mpsc};
use tracing::info;

mod hub;

#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> Result<UbiquiTimesService, shuttle_runtime::Error> {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    Ok(UbiquiTimesService { secret_store, pool })
}

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct UbiquiTimesService {
    secret_store: SecretStore,
    pool: PgPool,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for UbiquiTimesService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // Start your service and bind to the socket address
        println!("addr: {:?}", _addr);

        let discord_token = self
            .secret_store
            .get("DISCORD_TOKEN")
            .context("'DISCORD_TOKEN' was not found")?;

        let (tx, mut rx) = mpsc::channel(100);

        let discord_arg = DiscordArg {
            discord_bot_token: discord_token,
            pool: self.pool.clone(),
            channel: tx,
        };
        let discord_bot = tokio::spawn(start_discord_bot(discord_arg));

        let slack_bot = tokio::spawn(start_slack_bot());

        let (broad_tx, mut broad_rx) = broadcast::channel(100);
        let queue = tokio::spawn(hub::queue(rx, broad_tx));

        tokio::select! {
            _ = discord_bot => {
                info!("Discord bot finished");
            },
            _ = slack_bot => {
                info!("Slack bot finished");
            },
            _ = queue => {
                info!("Queue finished");
            },
        }
        Ok(())
    }
}
