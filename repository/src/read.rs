use share::model::PubMessage;

pub struct Repository {
    pool: sqlx::PgPool,
}

impl Repository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    /// PubMessageを要求しているが，必要なのはその中のユーザーIDだけ
    pub fn read(msg: PubMessage) {
        todo!("任意のユーザーIDから，他のアプリのユーザーIDも取得する．それらすべてに大して取得クエリを投げる スキーマのユーザー実装後にこの関数を実装する");
        match msg {
            PubMessage::Discord(msg) => {
                let user_id = msg.user_id;
            }
            PubMessage::Slack => {}
        }
    }

    fn discord_read(&self, user_id: u64) {
        // let _query = sqlx::query(
        //     r"
        //     SELECT * FROM discord_times WHERE user_id = $1",
        // )
        // .bind(user_id)
        // .fetch_all(&self.pool)
        // .await?;
    }
}
