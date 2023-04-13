use crate::db::DatabaseManager;

/// Add currency command to user currency list in db
///
/// # Arguments
///
/// * `msg` - Message
/// * `currency` - Currency
/// * `db` - DatabaseManager
///
/// # Returns
///
/// * `String` - Response message
pub async fn add_currency_command(user_id: i64, currency: String, db: DatabaseManager) -> String {
    let result = db.change_user_currency(user_id, currency.clone()).await;
    match result {
        Ok(()) => format!("Добавили валюту {:?}", currency),
        Err(err) => err.to_string(),
    }
}

/// Remove currency command from user currency list in db
///
/// # Arguments
///
/// * `msg` - Message
/// * `currency` - Currency
/// * `db` - DatabaseManager
///
/// # Returns
///
/// * `String` - Response message

pub async fn remove_currency_command(
    user_id: i64,
    currency: String,
    db: DatabaseManager,
) -> String {
    let result = db.remove_user_currency(user_id, currency.clone()).await;
    match result {
        Ok(()) => format!("Удалили валюту {:?}", currency),
        Err(err) => err.to_string(),
    }
}
