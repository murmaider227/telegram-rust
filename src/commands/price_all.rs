use crate::models::user::User;
use log::info;
use reqwest::Url;
use std::env;

pub async fn price_all_command(user: User) -> String {
    info!("price_all_command");
    if user.currency.is_empty() {
        return "You don't have any currency, type /addcurency curency-name".to_string();
    }
    let result = get_currency_price_multi(user.currency).await;
    match result {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}

async fn get_currency_price_multi(
    currency: Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let currency_string = currency.clone().join(",").to_uppercase();

    let token = env::var("CMC_TOKEN").expect("Fatality! CMC_PRO_API_KEY not set!");

    let url = Url::parse_with_params(
        "https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest",
        &[("symbol", &currency_string)],
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
            currency_string,
            response.status().as_u16()
        )
        .into());
    }

    let response_json = response.json::<serde_json::Value>().await;

    let response_json = match response_json {
        Ok(response_json) => response_json,
        Err(err) => return Err(err.into()),
    };

    let mut result_vec = Vec::new();

    for item in currency {
        let price =
            response_json["data"][&item.to_uppercase()][0]["quote"]["USD"]["price"].as_f64();
        let price_str = price.map(|price| {
            format!(
                "Coin📈: {}\nPrice USD💵: {:.2}$\n",
                item.to_uppercase(),
                price
            )
        });
        if let Some(price_str) = price_str {
            result_vec.push(price_str);
        }
    }

    let result_string = result_vec.join("-----------\n");

    Ok(result_string)
}
