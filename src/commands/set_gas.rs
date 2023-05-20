use crate::db::DatabaseManager;

pub async fn set_gas_command(
    user_id: i64,
    cfg: DatabaseManager,
    blockchain: String,
    gas: u32,
) -> String {
    let blockchain = match blockchain.to_lowercase().as_str() {
        "ethereum" => "ethereum".to_string(),
        "eth" => "ethereum".to_string(),
        "bsc" => "bsc".to_string(),
        "binance" => "bsc".to_string(),
        "binance smart chain" => "bsc".to_string(),
        "polygon" => "polygon".to_string(),
        "matic" => "polygon".to_string(),
        _ => return "Данный блокчейн не поддерживаеться".to_string(),
    };
    let user_query = cfg.get_user(user_id).await;
    let mut user = match user_query {
        Some(user) => user,
        None => return "Error getting user".to_string(),
    };
    user.gas.insert(blockchain.clone(), gas);
    let result = cfg.update_user(user).await;
    match result {
        Ok(res) => res,
        Err(err) => return err.to_string(),
    }
    format!(
        "Уведомления для сети {} успешно установлены на {} gwei",
        blockchain, gas
    )
}
