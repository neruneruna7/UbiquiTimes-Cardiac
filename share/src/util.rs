///  Ubiquitimesからの拡散だとわかるように，ユーザー名にプレフィックスを付加する
pub fn ubiquitimes_user_name(user_name: String) -> String {
    const UBIQUITIMES_USER_NAME_PREFIX: &str = "UT-";
    format!("{}{}", UBIQUITIMES_USER_NAME_PREFIX, user_name)
}
