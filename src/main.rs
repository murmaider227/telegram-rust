use teloxide::prelude::*;

use dotenvy::dotenv;
use env_logger::{Builder, Env};

mod commands;
mod db;
mod handlers;
mod models;
mod tests;

use crate::db::DatabaseManager;
use log::info;
use std::env::var;

use crate::handlers::currency::register_currency_handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting command bot...");

    let bot = Bot::from_env();

    let db = connect_to_db().await;

    register_currency_handlers(bot.clone(), db.clone()).await;
}

async fn connect_to_db() -> DatabaseManager {
    let db_user: String = var("db_user").expect("db_user must be set");
    let db_password: String = var("db_password").expect("db_password must be set");
    let db_host: String = var("db_host").expect("db_host must be set");
    let db_name: String = var("db_name").expect("db_name must be set");

    let uri = format!("mongodb://{}:{}@{}:27017", db_user, db_password, db_host);
    info!("DB_URL: {}", uri);

    DatabaseManager::new(&uri, &db_name)
        .await
        .expect("Failed to connect to MongoDB")
}
