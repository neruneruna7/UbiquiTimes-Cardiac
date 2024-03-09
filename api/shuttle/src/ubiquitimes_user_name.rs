const UBIQUITIMES_USER_NAME_PREFIX: &str = "UT-";

///  Ubiquitimesからの拡散だとわかるように，ユーザー名にプレフィックスを付加する
pub fn ubiquitimes_user_name(user_name: String) -> String {
    format!("{}{}", UBIQUITIMES_USER_NAME_PREFIX, user_name)
}
