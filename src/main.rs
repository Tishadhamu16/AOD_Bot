// fn main() {
//     println!("Hello, world!");
// }

mod abis;
mod arbitrage;
mod config;
mod constants;
mod dex;
mod ethereum;
mod types;
mod database;

use config::Config;
use ethereum::create_client;
use dex::get_all_prices;
use arbitrage::find_arbitrage_opportunities;
use std::process;
use ethers::middleware::Middleware;
use std::time::Duration;
use tokio::time::sleep;
use database::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initializing logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();
    
    // Loading configuration
    let config = Config::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {}", e);
        process::exit(1);
    });
    
    log::info!("Starting arbitrage bot...");
    
    // Create Ethereum client
    let client = match create_client(&config.polygon_rpc_url).await {
        Ok(client) => client,
        Err(e) => {
            log::error!("Failed to create Ethereum client: {}", e);
            log::error!("Please check your RPC URL and internet connection");
            log::error!("Current RPC URL: {}", config.polygon_rpc_url);
            process::exit(1);
        }
    };
    
    // Test the connection
    match client.get_block_number().await {
    Ok(block_number) => {
        log::info!("Connected to Polygon network (Current block: {})", block_number);
    },
    Err(e) => {
        log::error!("Failed to verify connection: {}", e);
        process::exit(1);
    }
}

    // database connection
    let db = match Database::new(&config.database_url).await {
        Ok(db) => db,
        Err(e) => {
            log::error!("Failed to connect to database: {}", e);
            log::error!("Please check your DATABASE_URL: {}", config.database_url);
            process::exit(1);
        }
    };

    log::info!("Connected to the database");
    
    log::info!("Arbitrage bot started with config: {:?}", config);
    
    // Main polling loop
    loop {
        log::info!("Fetching prices...");
        
        match get_all_prices(&client, config.trade_amount).await {
            Ok(prices) => {
                if prices.is_empty() {
                    log::warn!("No prices fetched from any DEX");
                    continue;
                }
                
                log::info!("Fetched prices from {} DEXes", prices.len());
                
                for price in &prices {
                    log::info!("{} price: {:.2} USDC", price.dex_name, price.price);
                }
                
                // Check for arbitrage opportunities
                let opportunities = find_arbitrage_opportunities(
                    &prices,
                    config.trade_amount,
                    config.min_profit_threshold,
                );
                
                if !opportunities.is_empty() {
                    log::info!("Found {} arbitrage opportunities:", opportunities.len());
                    
                    for opportunity in opportunities {
                        log::info!(
                            "ARBITRAGE: Buy on {} at {:.2}, Sell on {} at {:.2}, Profit: {:.2} USDC",
                            opportunity.buy_dex,
                            opportunity.buy_price,
                            opportunity.sell_dex,
                            opportunity.sell_price,
                            opportunity.profit
                        );

                        // Store in  database
                        // if let Err(e) = db.store_opportunity(&opportunity).await {
                        //     log::error!("Failed to store opportunity in database: {}", e);
                        // }
                    }
                } else {
                    log::info!("No arbitrage opportunities found");
                }
            }
            Err(e) => {
                log::error!("Failed to fetch prices: {}", e);
            }
        }
        
        // Wait before next poll
        sleep(Duration::from_millis(config.poll_interval_ms)).await;
    }
}