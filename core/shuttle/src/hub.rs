//! DiscordやSlackのPublisherから送られてくるデータを集約する
//! ここからSubscriberにデータを送る

use share::model::{PubMessage, Times};
use tokio::sync::{broadcast::Sender, mpsc::Receiver};
use tracing::info;

#[tracing::instrument(skip(receiver, sender))]
pub async fn queue(
    mut receiver: Receiver<PubMessage>,
    sender: Sender<PubMessage>,
) -> anyhow::Result<()> {
    info!("queue start");
    loop {
        let times = receiver.recv().await;
        todo!("DBから必要なTimes情報をReadする");

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
