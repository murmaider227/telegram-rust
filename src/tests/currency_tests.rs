use crate::commands::price::price_command;
use crate::commands::price_all::price_all_command;
use crate::models::user::User;
use dotenvy::dotenv;
//use super::*;

#[tokio::test]
async fn test_price_command_invalid_currency() {
    dotenv().ok();
    let value = 1.0;
    let currency = "CURRENCY".to_string();
    let result = price_command(value, currency).await;
    assert_eq!(
        result,
        "Error fetching price for CURRENCY: Currency not found"
    );
}

#[tokio::test]
async fn test_price_command_invalid_value() {
    let value = 0.0;
    let currency = "BTC".to_string();
    let result = price_command(value, currency).await;
    assert_eq!(result, "Error invalid value: 0.0");
}

#[tokio::test]
async fn test_price_all_command_contain() {
    dotenv().ok();
    let user = User::new(
        1,
        "".to_string(),
        vec!["BTC".to_string(), "ETH".to_string()],
    );
    let result = price_all_command(user).await;
    assert!(result.to_lowercase().contains("btc"));
}

#[tokio::test]
async fn test_price_all_command_not_contain() {
    dotenv().ok();
    let user = User::new(
        1,
        "".to_string(),
        vec!["BTC".to_string(), "NOTREALCURRENCY".to_string()],
    );
    let result = price_all_command(user).await;
    assert!(!result.to_lowercase().contains("NOTREALCURRENCY"));
}

#[tokio::test]
async fn test_price_all_command_empty() {
    let user = User::new(1, "".to_string(), vec![]);
    let result = price_all_command(user).await;
    assert_eq!(
        result,
        "You don't have any currency, type /addcurency curency-name"
    );
}
