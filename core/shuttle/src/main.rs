use anyhow::Context as _;
use discord::start_discord_bot;
use shuttle_runtime::{CustomError, SecretStore};
use slack::start_slack_bot;
use sqlx::{Executor as _, PgPool};
use tracing::info;

#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> Result<UbiquiTimesService, shuttle_runtime::Error> {
    pool.execute(include_str!("../../db/schema.sql"))
    .await
    .map_err(CustomError::new)?;

    Ok(UbiquiTimesService {
        secret_store,
    })
}

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct UbiquiTimesService {
    secret_store: SecretStore,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for UbiquiTimesService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // Start your service and bind to the socket address
        println!("addr: {:?}", _addr);

        let _discord_token = self
            .secret_store
            .get("DISCORD_TOKEN")
            .context("'DISCORD_TOKEN' was not found")?;

        let discord_arg = discord::DiscordArg {
            discord_bot_token: _discord_token,
        };
        let discord_bot = tokio::spawn(start_discord_bot(discord_arg));

        let slack_bot = tokio::spawn(start_slack_bot());

        tokio::select! {
            _ = discord_bot => {
                info!("Discord bot finished");
            },
            _ = slack_bot => {
                info!("Slack bot finished");
            },
        }
        Ok(())
    }
}
