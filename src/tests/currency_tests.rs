use crate::commands::price::price_command;
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
