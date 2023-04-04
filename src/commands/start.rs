use crate::db::DatabaseManager;
use crate::models::user::User;
use log::debug;
use teloxide::prelude::Message;

/// Start command
///
/// # Arguments
///
/// * `msg` - Message
///
/// # Returns
///
/// * `String` - Response message
pub async fn start_command(msg: Message, cfg: DatabaseManager) -> String {
    let res = User::new(
        msg.from().unwrap().id.0 as i64,
        msg.from().unwrap().username.clone().unwrap(),
        vec![],
    );
    let save = cfg.insert_user(res).await;
    match save {
        Ok(_) => (),
        Err(_) => debug!("user already exists"),
    }
    "Hello with start".to_string()
}
