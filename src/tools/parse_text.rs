use crate::tools::parse_currency::parse_currency;
use crate::tools::parse_twitter::parse_twitter_links;

pub async fn parse_text(text: &str) -> String {
    let mut result = String::new();
    result += &parse_currency(text).await.unwrap_or_default();
    result += &*"\n".to_string();
    result += &parse_twitter_links(text).await.unwrap_or_default();

    result
}
