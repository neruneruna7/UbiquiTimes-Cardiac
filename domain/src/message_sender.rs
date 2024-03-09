use crate::repository::UtTime;

#[derive(Debug, Clone)]
pub struct TimesMessage {
    pub avater_url: String,
    pub content: String,
}

pub trait TimesMessageSender {
    type Error;
    async fn send_all(&self, message: TimesMessage, times: Vec<UtTime>) -> Result<(), Self::Error>;
}
