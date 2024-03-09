pub struct TimesMessage {
    pub content: String,
}

pub trait TimesMessageSender {
    type Error;
    fn send_message(&self, message: TimesMessage) -> Result<(), Self::Error>;
}
