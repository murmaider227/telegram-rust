use reqwest::Url;
use std::env;
pub async fn price_command(value: f64, currency: String) -> String {
    let result = get_currency_price(value, currency).await;
    match result {
        Ok(result) => result,
        Err(err) => err.to_string(),
    }
}

/// Fetches the price of a cryptocurrency from CoinMarketCap
///
/// # Arguments
///
/// * `value` - The amount of the currency to convert
/// * `currency` - The currency to convert to
///
/// # Returns
///
/// * `Result<String, Box<dyn std::error::Error>>` - The price of the currency in USD
///
/// # Errors
///
/// * `Error invalid value: 0.0` - If the value is 0.0
/// * `Error fetching price for {}: Status code {}` - If the status code is not 200
/// * `Error fetching price for {}: Currency not found` - If the currency is not found
///
/// # Examples
///
/// ```
/// let result = get_currency_price(value, currency).await;
/// match result {
/// Ok(result) => result,
/// Err(err) => err.to_string(),
/// }
///
/// ```
///

async fn get_currency_price(
    value: f64,
    currency: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    if value == 0.0 {
        return Err("Error invalid value: 0.0".to_string().into());
    }

    let currency = currency.to_uppercase();

    let token = env::var("CMC_TOKEN").expect("Fatality! CMC_PRO_API_KEY not set!");

    let url = Url::parse_with_params(
        "https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest",
        &[("symbol", &currency)],
    )?;

    let response = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", token)
        .header("Accept", "application/json")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!(
            "Error fetching price for {}: Status code {}",
            currency,
            response.status().as_u16()
        )
        .into());
    }

    let response_json = response.json::<serde_json::Value>().await;

    let response_json = match response_json {
        Ok(response_json) => response_json,
        Err(err) => return Err(err.into()),
    };

    let price = response_json["data"][&currency][0]["quote"]["USD"]["price"].as_f64();

    let result = match price {
        Some(price) => format!("The price of {} is ${:.2}", currency, price * value),
        None => format!("Error fetching price for {}: Currency not found", currency),
    };

    Ok(result)
}
