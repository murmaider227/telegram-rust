use crate::db::DatabaseManager;
use log::debug;
use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    /// User id
    pub user_id: i64,
    /// User username
    pub username: String,
    /// User currency
    pub currency: Vec<String>,
    /// User created at
    pub created_at: bson::DateTime,
    /// User updated at
    pub updated_at: bson::DateTime,
    /// User notification
    pub notification: bool,
    /// User gas
    pub gas: HashMap<String, u32>,
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
            notification: false,
            gas: HashMap::new(),
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

    pub async fn update(&self, db: DatabaseManager) -> Result<(), Box<dyn Error>> {
        let res = db.update_user(self.clone()).await;
        match res {
            Ok(_) => (),
            Err(err) => debug!("update error {}", err),
        }
        Ok(())
    }
}
