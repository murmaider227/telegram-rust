use regex::Regex;
use serde_json::{Map, Value};
use std::env;
use std::error::Error;

pub async fn parse_currency(text: &str) -> Option<String> {
    let re = match Regex::new(r"([0-9.,]+)\s+([a-zA-Z]+)") {
        Ok(regex) => regex,
        Err(_) => return None,
    };
    let text = text.to_uppercase();
    let result = re.captures_iter(&text);
    let mut coins = Vec::new();

    for cap in result {
        let amount = cap.get(1)?.as_str().to_owned();
        let currency = cap.get(2)?.as_str().to_owned();
        coins.push((amount, currency));
    }

    if !coins.is_empty() {
        Some(parser_coins_mult(&coins).await.ok()?) // Вернуть None, если parser_coins_mult вернет ошибку
    } else {
        None
    }
}

async fn get_mult_value(coins: &[(String, String)]) -> Result<Map<String, Value>, Box<dyn Error>> {
    let url = "https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest";
    let blacklist = &["NFT", "USD"];
    let crypto_symbols: Vec<&str> = coins
        .iter()
        .map(|(_, crypto)| crypto.as_str())
        .filter(|crypto| !blacklist.contains(crypto))
        .collect();

    let token = env::var("CMC_TOKEN").expect("Fatality! CMC_PRO_API_KEY not set!");

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .query(&[("symbol", &crypto_symbols.join(","))])
        .header("X-CMC_PRO_API_KEY", token)
        .header("Accept", "application/json")
        .send()
        .await?;

    let response_json = response.json::<Value>().await?;
    let prices_data = response_json["data"]
        .as_object()
        .ok_or("Response does not contain valid prices data")?
        .clone();

    Ok(prices_data)
}

async fn parser_coins_mult(coins: &[(String, String)]) -> Result<String, Box<dyn Error>> {
    let prices_data = get_mult_value(coins).await?;

    let result = coins
        .iter()
        .filter_map(|(amount, crypto)| {
            prices_data.get(crypto).and_then(|price_data| {
                let price = price_data[0]["quote"]["USD"]["price"].as_f64()?;
                let amount_usd = amount.parse::<f64>().ok()? * price;
                Some(format!("💰{} {}\n{:.4} usd\n", amount, crypto, amount_usd))
            })
        })
        .collect::<Vec<_>>()
        .join("");

    Ok(result)
}
