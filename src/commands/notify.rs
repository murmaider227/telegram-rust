use crate::db::DatabaseManager;

pub async fn notify_command(user_id: i64, cfg: DatabaseManager) -> String {
    let user_query = cfg.get_user(user_id).await;
    let user = match user_query {
        Some(user) => user,
        None => return "Error getting user".to_string(),
    };
    let result = cfg.change_notify(user).await;
    match result {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}
