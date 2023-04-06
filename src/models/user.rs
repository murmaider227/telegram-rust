use crate::db::DatabaseManager;
use log::debug;
use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// User model
///
/// # Fields
///
/// * `user_id` - User id
/// * `username` - User username
/// * `currency` - User currency
/// * `created_at` - User created at
/// * `updated_at` - User updated at
///
/// # Methods
///
/// * `new` - Create new user
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub currency: Vec<String>,
    pub created_at: bson::DateTime,
    pub updated_at: bson::DateTime,
}

impl User {
    /// Create new user
    ///
    /// # Arguments
    ///
    /// * `user_id` - User id
    /// * `username` - User username
    /// * `currency` - User currency
    ///

    pub fn new(user_id: i64, username: String, currency: Vec<String>) -> Self {
        Self {
            user_id,
            username,
            currency,
            created_at: bson::DateTime::now(),
            updated_at: bson::DateTime::now(),
        }
    }

    pub async fn save(&self, db: DatabaseManager) -> Result<(), Box<dyn Error>> {
        let res = db.insert_user(self.clone()).await;
        match res {
            Ok(_) => (),
            Err(_) => debug!("user already exists"),
        }
        Ok(())
    }
}
