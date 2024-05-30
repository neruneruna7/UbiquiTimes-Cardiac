use crate::models::UtTime;

pub trait TimesMessageSender {
    type Error;
    type Message;
    // テキストは別途用意する
    // コマンドの引数としてわたってくるから，それを使う
    fn send_all(
        &self,
        message: &Self::Message,
        text: String,
        times: Vec<UtTime>,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
}
