use ethers::{
    providers::{Http, Provider},
    middleware::Middleware
};
use std::sync::Arc;
use anyhow::Result;
use std::time::Duration;

pub type EthereumClient = Arc<Provider<Http>>;

// Creates a connected Ethereum client for Polygon
pub async fn create_client(rpc_url: &str) -> Result<EthereumClient> {
    log::debug!("Connecting to Polygon RPC: {}", rpc_url);
    
    let provider = Provider::<Http>::try_from(rpc_url)
        .map_err(|e| anyhow::anyhow!("Failed to create provider: {}", e))?;

    Ok(Arc::new(provider))
}