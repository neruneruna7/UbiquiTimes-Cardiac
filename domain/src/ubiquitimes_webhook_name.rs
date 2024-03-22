/// Weebhook名はUT-c_{user_id}とする
pub const WEBHOOK_NAME_PREFIX: &str = "UT-c_";

pub fn webhook_name(user_id: u64) -> String {
    let webhook_name = format!("{}{}", WEBHOOK_NAME_PREFIX, user_id);

    webhook_name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_name() {
        let user_id = 123456789;
        let expected = "UT-c_123456789".to_string();
        assert_eq!(webhook_name(user_id), expected);
    }
}