const UBIQUITIMES_USER_NAME_PREFIX: &str = "UT-";

///  Ubiquitimesからの拡散だとわかるように，ユーザー名にプレフィックスを付加する
pub fn ubiquitimes_user_name(user_name: String) -> String {
    format!("{}{}", UBIQUITIMES_USER_NAME_PREFIX, user_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ubiquitimes_user_name() {
        let user_name = "user_name".to_string();
        let expected = "UT-user_name".to_string();
        assert_eq!(ubiquitimes_user_name(user_name), expected);
    }
}
