use crate::tools::parse_currency::parse_currency;
use crate::tools::parse_eden::parse_eden_command;
//use crate::tools::parse_twitter::parse_twitter_links;

pub async fn parse_text(text: &str) -> String {
    let mut result = String::new();
    result += &parse_currency(text).await.unwrap_or_default(); // parse currency from CoinMarketCap API
    // result += &parse_twitter_links(text).await.unwrap_or_default(); // parse twitter links
    result += &*parse_eden_command(text).await.unwrap_or_default(); // Parse collections from magic eden

    result
}
