use crate::db::DatabaseManager;
use crate::models::user::User;
use log::debug;
use teloxide::prelude::Message;

pub async fn start_command(msg: Message, cfg: DatabaseManager) -> String {
    let res = User::new(
        msg.from().unwrap().id.0 as i64,
        msg.from()
            .unwrap()
            .username
            .clone()
            .unwrap_or("".to_string()),
        vec![],
    );
    let save = cfg.insert_user(res).await;
    match save {
        Ok(_) => (),
        Err(err) => debug!("user already exists: {}", err),
    }
    "Hello type /help".to_string()
}
