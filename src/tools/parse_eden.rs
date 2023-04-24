use crate::models::errors::CustomError;
use log::{debug, error};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub async fn parse_eden_command(text_to_parse: &str) -> Option<String> {
    let res = parse_eden_link(text_to_parse).await;

    match res {
        Ok(text) => Some(text),
        Err(e) => {
            debug!("Eden Error: {}", e);
            None
        }
    }
}

/// Parses the eden link and returns the text to be sent to the chat
async fn parse_eden_link(text_to_parse: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"(?:http|https)://(?:www\.)?magiceden\.io/[^? ]+")?;
    let result = re.find_iter(text_to_parse);

    let client = Client::new();

    let mut text = String::new();
    for item in result {
        let parts: Vec<&str> = item.as_str().split('/').collect();
        if let Some(part) = parts.last() {
            match parse_eden(part.to_string(), &client).await {
                Ok(parsed_text) => {
                    text += &parsed_text;
                }
                Err(e) => {
                    error!("Error parsing eden link: {}", e);
                }
            }
        } else {
            error!("Couldn't get the last part of the URL");
        }
    }

    Ok(text)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Listing {
    pda_address: String,
    auction_house: String,
    token_address: String,
    token_mint: String,
    seller: String,
    token_size: u32,
    price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Collection {
    symbol: String,
    floor_price: f64,
    listed_count: u32,
    volume_all: f64,
}

async fn parse_eden(collection: String, client: &Client) -> Result<String, CustomError> {
    debug!("Parsing eden link: {}", collection);
    let json_stats: Collection = get_eden_stats(collection.clone(), client).await?;
    let json: Vec<Listing> = get_eden_prices(collection, client, json_stats.listed_count).await?;

    Ok(sort_eden(&json, json_stats.floor_price).await)
}

/// Gets price data from eden
async fn get_eden_prices(
    collection: String,
    client: &Client,
    listed_count: u32,
) -> Result<Vec<Listing>, CustomError> {
    let mut listings: Vec<Listing> = Vec::with_capacity((listed_count + 10) as usize); // Add 10 to the capacity to avoid reallocation's

    let mut offset = 0;
    debug!("Listed count: {}", listed_count);
    loop {
        let url = format!(
            "https://api-mainnet.magiceden.dev/v2/collections/{}/listings?offset={}&limit=20",
            collection, offset
        );

        let response = client.get(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            return Err(CustomError::HttpStatus(status));
        }
        let mut json: Vec<Listing> = serde_json::from_str(&response.text().await?)?;

        listings.append(&mut json);
        if offset + 20 > listed_count {
            break;
        }

        offset += 20;
    }

    Ok(listings)
}

/// Gets stats from eden
async fn get_eden_stats(collection: String, client: &Client) -> Result<Collection, CustomError> {
    let url = format!(
        "https://api-mainnet.magiceden.dev/v2/collections/{}/stats",
        collection
    );

    let response = client.get(&url).send().await?;

    let status = response.status();
    if !status.is_success() {
        return Err(CustomError::HttpStatus(status));
    }

    Ok(serde_json::from_str(&response.text().await?)?)
}

/// Sorts the eden prices into 5 categories
async fn sort_eden(data: &[Listing], floor_gwei: f64) -> String {
    let floor = floor_gwei / 1_000_000_000.0; // convert to sol

    // Initialize the variables for each category
    let [mut low_price, mut sol_price, mut solhalf_price, mut two_sol_price, mut max_price] =
        [0; 5];

    // Determine the change in price between each category
    let change = match floor {
        floor if floor <= 0.5 => 0.25,
        floor if floor < 1.0 => 0.5,
        floor if floor < 2.0 => 1.0,
        floor if floor < 5.0 => 2.0,
        floor if floor < 10.0 => 5.0,
        _ => 10.0,
    };
    // Sort the prices into the categories
    for item in data.iter() {
        match item.price {
            price if price < floor + change => low_price += 1,
            price if price < floor + change * 2.0 => sol_price += 1,
            price if price < floor + change * 3.0 => solhalf_price += 1,
            price if price < floor + change * 4.0 => two_sol_price += 1,
            _ => max_price += 1,
        }
    }
    // Return the text to be sent to the chat
    format!(
        "💎Флор: {:.5}\nПредметов ценой до {:.5} sol: {}\nот {:.5} sol до {:.5} sol: {}\nот {:.5} sol до {:.5} sol: {}\nот {:.5} sol до {:.5} sol: {}\nБольше {:.5} sol: {}\n",
        floor,
        floor + change,
        low_price,
        floor + change,
        floor + change * 2.0,
        sol_price,
        floor + change * 2.0,
        floor + change * 3.0,
        solhalf_price,
        floor + change * 3.0,
        floor + change * 4.0,
        two_sol_price,
        floor + change * 4.0,
        max_price,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sort_eden() {
        let listings = vec![
            Listing {
                pda_address: String::from("1"),
                auction_house: String::from("eden"),
                token_address: String::from("2"),
                token_mint: String::from("3"),
                seller: String::from("4"),
                token_size: 5,
                price: 1.0, // 1 SOL
            },
            Listing {
                pda_address: String::from("6"),
                auction_house: String::from("eden"),
                token_address: String::from("7"),
                token_mint: String::from("8"),
                seller: String::from("9"),
                token_size: 10,
                price: 2.5, // 2.5 SOL
            },
            Listing {
                pda_address: String::from("11"),
                auction_house: String::from("eden"),
                token_address: String::from("12"),
                token_mint: String::from("13"),
                seller: String::from("14"),
                token_size: 15,
                price: 5.0, // 5 SOL
            },
            Listing {
                pda_address: String::from("16"),
                auction_house: String::from("eden"),
                token_address: String::from("17"),
                token_mint: String::from("18"),
                seller: String::from("19"),
                token_size: 20,
                price: 8.0, // 8 SOL
            },
            Listing {
                pda_address: String::from("21"),
                auction_house: String::from("eden"),
                token_address: String::from("22"),
                token_mint: String::from("23"),
                seller: String::from("24"),
                token_size: 25,
                price: 12.0, // 12 SOL
            },
        ];
        let expected_output = "💎Флор: 1.00000\nПредметов ценой до 2.00000 sol: 1\nот 2.00000 sol до 3.00000 sol: 1\nот 3.00000 sol до 4.00000 sol: 0\nот 4.00000 sol до 5.00000 sol: 0\nБольше 5.00000 sol: 3\n";
        assert_eq!(sort_eden(&listings, 1000000000.0).await, expected_output);
    }

    #[tokio::test]
    async fn test_get_eden_stats() {
        let client = Client::new();
        let json: Collection = get_eden_stats("degods".to_string(), &client).await.unwrap();
        assert_eq!(json.symbol, "degods");
    }

    #[tokio::test]
    async fn test_invslid_get_eden_listings() {
        let client = Client::new();
        let json: Vec<Listing> = get_eden_prices("InvalidCollection2929".to_string(), &client, 10)
            .await
            .unwrap();
        // result is error
        assert_eq!(json.len(), 0);
    }

    #[tokio::test]
    async fn test_get_eden_listings() {
        let client = Client::new();
        let json: Vec<Listing> = get_eden_prices("degods".to_string(), &client, 10)
            .await
            .unwrap();
        assert!(!json.is_empty());
    }
}
