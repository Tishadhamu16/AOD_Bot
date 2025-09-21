use ethers::{
    prelude::*,
    types::Address,
};
use std::sync::Arc;
use anyhow::Result;
use crate::abis::IUniswapV2Router02;
use crate::types::DexPrice;
use crate::constants::*;

pub async fn get_all_prices(
    client: &Arc<Provider<Http>>,
    trade_amount: f64,
) -> Result<Vec<DexPrice>> {
    // Parsing addresses from strings
    let weth_addr: Address = parse_address(WETH_ADDRESS_STR)?;
    let usdc_addr: Address = parse_address(USDC_ADDRESS_STR)?;
    let quickswap_router: Address = parse_address(QUICKSWAP_ROUTER_ADDRESS_STR)?;
    let sushiswap_router: Address = parse_address(SUSHISWAP_ROUTER_ADDRESS_STR)?;
    
    // Convert trade amount to the appropriate units
    let amount_in = U256::from((trade_amount * 1e18) as u128);
    
    let mut prices = Vec::new();
    
    // Get price from QuickSwap
    match get_dex_price(client, quickswap_router, "QuickSwap", weth_addr, usdc_addr, amount_in).await {
        Ok(price) => prices.push(price),
        Err(e) => log::error!("Failed to get QuickSwap price: {}", e),
    }
    
    // Get price from SushiSwap
    match get_dex_price(client, sushiswap_router, "SushiSwap", weth_addr, usdc_addr, amount_in).await {
        Ok(price) => prices.push(price),
        Err(e) => log::error!("Failed to get SushiSwap price: {}", e),
    }
    
    Ok(prices)
}

// function to get price form particular dex
pub async fn get_dex_price(
    client: &Arc<Provider<Http>>,
    router_address: Address,
    dex_name: &str,
    token_in: Address,
    token_out: Address,
    amount_in: U256,
) -> Result<DexPrice> {
    // Creating a contract instance for the DEX router
    let router = IUniswapV2Router02::new(router_address, client.clone());
    
    // token path: [input_token, output_token]
    let path = vec![token_in, token_out];
    
    log::debug!("Calling get_amounts_out with amount_in: {}", amount_in);
    // Call the getAmountsOut function on the router contract
    let amounts: Vec<U256> = router 
        .get_amounts_out(amount_in, path)
        .call()
        .await?;
    
    log::debug!("Received amounts from {}: {:?}", dex_name, amounts);
    
    let amount_out = amounts[1].as_u128() as f64;
    
    // Calculate price per token 
    let amount_in_f64 = amount_in.as_u128() as f64;

    let price = if amount_in_f64 > 0.0 {
    (amount_out / 1_000_000.0) / (amount_in_f64 / 1_000_000_000_000_000_000.0)
    } else {
    0.0
    };
    
    log::debug!("amount in: {}price: {}", amount_in, price);

    Ok(DexPrice {
        dex_name: dex_name.to_string(),
        price,
        router_address,
    })
}