use teloxide::prelude::*;

use dotenvy::dotenv;
mod commands;
mod db;
mod handlers;
mod models;
mod tests;
mod tools;

use flexi_logger::{colored_opt_format, opt_format, FileSpec, Logger};

use crate::db::DatabaseManager;
use log::*;
use std::env::var;

use crate::handlers::currency::register_currency_handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();
    Logger::try_with_env()
        .unwrap()
        .format(opt_format)
        .format_for_stderr(colored_opt_format)
        .log_to_file(FileSpec::default().directory("logs"))
        .duplicate_to_stderr(flexi_logger::Duplicate::Debug)
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(7),
        )
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {:?}", e));

    info!("Starting command bot...");

    let bot = Bot::from_env();

    let db = connect_to_db().await;

    register_currency_handlers(bot.clone(), db.clone()).await;
}

async fn connect_to_db() -> DatabaseManager {
    let db_user: String = var("DB_USER").expect("db_user must be set");
    let db_password: String = var("DB_PASSWORD").expect("db_password must be set");
    let db_host: String = var("DB_HOST").expect("db_host must be set");
    let db_name: String = var("DB_NAME").expect("db_name must be set");

    let uri = format!("mongodb://{}:{}@{}:27017", db_user, db_password, db_host);

    DatabaseManager::new(&uri, &db_name)
        .await
        .expect("Failed to connect to MongoDB")
}
