//! DiscordやSlackのPublisherから送られてくるデータを集約する
//! ここからSubscriberにデータを送る

use share::model::Times;
use tokio::sync::{broadcast::Sender, mpsc::Receiver};
use tracing::info;

#[tracing::instrument(skip(receiver, sender))]
pub async fn queue(
    mut receiver: Receiver<Vec<Times>>,
    sender: Sender<Vec<Times>>,
) -> anyhow::Result<()> {
    loop {
        let times = receiver.recv().await;
        match times {
            Some(times) => {
                let r = sender.send(times);
                match r {
                    Ok(t) => info!("send times: {}", t),
                    Err(e) => info!("error: {:?}", e),
                }
            }
            None => info!("QueueReceiver dropped"),
        };
    }
}
