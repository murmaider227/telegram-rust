use chrono::DateTime;
use chrono::Utc;
use log::{debug, error};
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::Value;
use std::{env, fmt};

pub async fn parse_twitter_links(text_to_parse: &str) -> Option<String> {
    let res = parse(text_to_parse).await;

    match res {
        Ok(Some(text)) => Some(text),
        Ok(None) => None,
        Err(e) => {
            debug!("Twitter Error: {}", e);
            None
        }
    }
}

async fn parse(text_to_parse: &str) -> Result<Option<String>, CustomError> {
    let re = Regex::new(r"https?://twitter.com/\w+")?;
    let result = re.find_iter(text_to_parse);

    let client = Client::new();

    let mut text = String::new();
    let mut found = false;
    for item in result {
        found = true;
        let parts: Vec<&str> = item.as_str().split('/').collect();
        if let Some(part) = parts.last() {
            match parse_twitter(part.to_string(), &client).await {
                Ok(parsed_text) => {
                    text += &parsed_text;
                }
                Err(e) => {
                    error!("Error parsing Twitter link: {}", e);
                }
            }
        } else {
            error!("Couldn't get the last part of the URL");
        }
    }

    if found {
        Ok(Some(text))
    } else {
        Ok(None)
    }
}

// Updated CustomError enum
#[derive(Debug)]
pub enum CustomError {
    Regex(regex::Error),
    Reqwest(reqwest::Error),
    InvalidHeader(reqwest::header::InvalidHeaderValue),
    HttpStatus(reqwest::StatusCode),
}

// Updated Display implementation for CustomError
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Regex(e) => write!(f, "Regex error: {}", e),
            CustomError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            CustomError::InvalidHeader(e) => write!(f, "Invalid header error: {}", e),
            CustomError::HttpStatus(e) => write!(f, "HTTP status error: {}", e),
        }
    }
}

// Update the From implementations for CustomError
impl From<regex::Error> for CustomError {
    fn from(err: regex::Error) -> CustomError {
        CustomError::Regex(err)
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError::Reqwest(err)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for CustomError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> CustomError {
        CustomError::InvalidHeader(err)
    }
}

async fn format_twitter(data: &serde_json::Map<String, Value>) -> String {
    // Try to parse the date and time from the input string
    let created_at = data["created_at"].as_str().and_then(|s| {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Utc))
            .ok()
    });

    // Format the date and time as desired, or use a default value if parsing failed
    let formatted_created_at = match created_at {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => "время не найдено".to_string(),
    };

    let text = format!(
        "📨Twitter \nНазвание: {}\nПодписчики: {}\nТвитов: {}\nСоздан: {}\n",
        data["username"],
        data["public_metrics"]["followers_count"],
        data["public_metrics"]["tweet_count"],
        formatted_created_at
    );
    text
}

async fn parse_twitter(name: String, client: &Client) -> Result<String, CustomError> {
    let url = format!(
        "https://api.twitter.com/2/users/by/username/{}?user.fields=public_metrics,created_at",
        name
    );

    let token = env::var("TWITTER_TOKEN").expect("Fatality! TWITTER_TOKEN not set!");
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );

    let response = client.get(&url).headers(headers).send().await?;
    let status = response.status();

    if !status.is_success() {
        return Err(CustomError::HttpStatus(status));
    }

    let json = response.json::<Value>().await?;
    if let Some(data) = json["data"].as_object() {
        let formatted_data = format_twitter(data).await;
        Ok(formatted_data)
    } else {
        Err(CustomError::HttpStatus(reqwest::StatusCode::BAD_REQUEST))
    }
}
