use crate::models::{TimesMessage, UtTime};

pub trait TimesMessageSender {
    type Error;
    type Message;
    // テキストは別途用意する
    // コマンドの引数としてわたってくるから，それを使う
    async fn send_all(
        &self,
        message: &Self::Message,
        text: String,
        times: Vec<UtTime>,
    ) -> Result<(), Self::Error>;
}
