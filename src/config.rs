use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub polygon_rpc_url: String,
    pub min_profit_threshold: f64,
    pub trade_amount: f64,
    pub poll_interval_ms: u64,
    pub database_url: String
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenv::dotenv().ok();
        
        Ok(Config {
            polygon_rpc_url: env::var("POLYGON_RPC_URL")
                .expect("POLYGON_RPC_URL must be set"),
            min_profit_threshold: env::var("MIN_PROFIT_THRESHOLD")
                .unwrap_or_else(|_| "10.0".to_string())
                .parse()
                .expect("MIN_PROFIT_THRESHOLD must be a valid number"),
            trade_amount: env::var("TRADE_AMOUNT")
                .unwrap_or_else(|_| "1.0".to_string())
                .parse()
                .expect("TRADE_AMOUNT must be a valid number"),
            poll_interval_ms: env::var("POLL_INTERVAL_MS")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .expect("POLL_INTERVAL_MS must be a valid number"),
            database_url: env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://username:password@localhost:5432/arbitrage_bot".to_string())    
        })
    }
}