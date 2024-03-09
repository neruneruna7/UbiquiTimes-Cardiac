use crate::models::{TimesMessage, UtTime};

pub trait TimesMessageSender {
    type Error;
    async fn send_all(&self, message: TimesMessage, times: Vec<UtTime>) -> Result<(), Self::Error>;
}
