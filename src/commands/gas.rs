use ethers::prelude::*;
use std::collections::HashMap;

pub async fn gas_command() -> String {
    let res = get_gas_all().await;
    match res {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}

async fn get_gas_all() -> Result<String, Box<dyn std::error::Error>> {
    let providers: HashMap<&str, &str> = [
        ("ethereum", "https://eth.llamarpc.com"),
        ("bsc", "https://bsc-dataseed.binance.org"),
        ("polygon", "https://polygon-bor.publicnode.com"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut result: String = "".to_string();

    for (blockchain, rpc_url) in providers.iter() {
        let provider = Provider::try_from(*rpc_url)?;
        let gas = get_gas(&provider).await?;
        result.push_str(&format!("{}: {} gwei\n", blockchain, gas));
    }

    Ok(result)
}

async fn get_gas(provider: &Provider<Http>) -> Result<String, Box<dyn std::error::Error>> {
    let gas_price_wei = provider.get_gas_price().await?;
    let gas_price = gas_price_wei / 1_000_000_000u128;
    Ok(gas_price.to_string())
}
