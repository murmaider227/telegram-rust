use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    Regex(regex::Error),
    Reqwest(reqwest::Error),
    InvalidHeader(reqwest::header::InvalidHeaderValue),
    HttpStatus(reqwest::StatusCode),
    Deserialize(serde_json::Error),
}

// Updated Display implementation for CustomError
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Regex(e) => write!(f, "Regex error: {}", e),
            CustomError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            CustomError::InvalidHeader(e) => write!(f, "Invalid header error: {}", e),
            CustomError::HttpStatus(e) => write!(f, "HTTP status error: {}", e),
            CustomError::Deserialize(e) => write!(f, "Deserialize error: {}", e),
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

impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> CustomError {
        CustomError::Deserialize(err)
    }
}
