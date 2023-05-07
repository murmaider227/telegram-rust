use crate::commands::price::price_command;
use crate::commands::price_all::price_all_command;
use crate::models::user::User;
use crate::tools::parse_currency::parse_currency;
use dotenvy::dotenv;
//use super::*;

#[tokio::test]
async fn test_price_command_invalid_currency() {
    dotenv().ok();
    let currency = "CURRENCY".to_string();
    let result = price_command(currency).await;
    assert_eq!(
        result,
        "Error fetching data for CURRENCY: Currency not found"
    );
}

// #[tokio::test]
// async fn test_price_command_invalid_value() {
//     let value = 0.0;
//     let currency = "BTC".to_string();
//     let result = price_command(value, currency).await;
//     assert_eq!(result, "Error invalid value: 0.0");
// }

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

#[tokio::test]
async fn test_parse_currency() {
    dotenv().ok();
    let text = "Hello, I want to buy 1 BTC";
    let result = parse_currency(text).await.unwrap();
    assert!(result.contains("BTC"));
}

#[tokio::test]
async fn test_parse_currency_with_multiple_currencies() {
    dotenv().ok();
    let text = "I have 0.5 BTC and 1000 ETH";
    let result = parse_currency(text).await.unwrap();
    assert!(result.contains("BTC"));
    assert!(result.contains("ETH"));
}

#[tokio::test]
async fn test_parse_currency_invalid_regex() {
    dotenv().ok();
    let text = "I have 1000BTC";
    let result = parse_currency(text).await;
    assert!(result.is_none());
}
