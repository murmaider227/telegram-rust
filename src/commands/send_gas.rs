use crate::db::DatabaseManager;
use ethers::prelude::*;
use log::debug;
use mongodb::bson::doc;
use std::collections::HashMap;
use teloxide::prelude::*;
use tokio::time::{interval, Duration};

pub async fn send_gas_all(bot: Bot, cfg: DatabaseManager) {
    let mut interval = interval(Duration::from_secs(60));
    tokio::spawn(async move {
        loop {
            interval.tick().await;

            let blockchain_gas = match get_gas_all().await {
                Ok(gas) => gas,
                Err(err) => {
                    log::error!("Error getting gas: {}", err);
                    return;
                }
            };
            match send_gas(bot.clone(), cfg.clone(), blockchain_gas).await {
                Ok(_) => (),
                Err(err) => {
                    log::error!("Error sending gas2: {}", err);
                    return;
                }
            }
        }
    });
}

async fn send_gas(
    bot: Bot,
    cfg: DatabaseManager,
    blockchain_gas: HashMap<String, U256>,
) -> Result<(), Box<dyn std::error::Error>> {
    for (blockchain, gas) in blockchain_gas.iter() {
        let filter = Some(doc! {
            "notification": true,
            format!("gas.{}", blockchain): { "$gt": (*gas).low_u64() as i64}
        });
        let users = cfg.get_all_users(filter).await?;

        for user in users {
            let message = format!("{}: {} Gwei\n", blockchain, gas);

            if let Err(err) = bot.send_message(UserId(user.user_id as u64), message).await {
                log::error!("Error sending gas: {}", err);
            }
        }
    }
    Ok(())
}

async fn get_gas_all() -> Result<HashMap<String, U256>, Box<dyn std::error::Error>> {
    let providers: HashMap<&str, &str> = [
        ("ethereum", "https://eth.llamarpc.com"),
        ("bsc", "https://bsc-dataseed.binance.org"),
        ("polygon", "https://polygon-bor.publicnode.com"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut result: HashMap<String, U256> = HashMap::new();

    for (blockchain, rpc_url) in providers.iter() {
        let provider = Provider::try_from(*rpc_url)?;
        let gas = get_gas(&provider).await?;
        result.insert(blockchain.to_string(), gas);
    }

    Ok(result)
}

async fn get_gas(provider: &Provider<Http>) -> Result<U256, Box<dyn std::error::Error>> {
    let gas_price_wei = provider.get_gas_price().await?;
    let gas_price = gas_price_wei / 1_000_000_000u128;
    Ok(gas_price)
}
