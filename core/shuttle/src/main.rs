use discord::start_discord_bot;
use slack::start_slack_bot;
use tracing::info;

#[shuttle_runtime::main]
async fn shuttle_main() -> Result<UbiquiTimesService, shuttle_runtime::Error> {
    Ok(UbiquiTimesService {})
}

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct UbiquiTimesService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for UbiquiTimesService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // Start your service and bind to the socket address
        println!("addr: {:?}", _addr);

        let discord_bot = tokio::spawn(start_discord_bot());
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
