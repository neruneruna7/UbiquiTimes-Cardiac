use crate::models::Context;

/// Weebhook名はUT-c_{user_id}とする
pub const WEBHOOK_NAME_PREFIX: &str = "UT-c_";

pub async fn webhook_name(ctx: Context<'_>) -> String {
    let user_id = ctx.author().id.get();
    let webhook_name = format!("{}{}", WEBHOOK_NAME_PREFIX, user_id);

    webhook_name
}
