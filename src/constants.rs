
use ethers::types::Address;

// storing addresses as strings and parse them when needed
// token addresses
pub const WETH_ADDRESS_STR: &str = "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619";
pub const USDC_ADDRESS_STR: &str = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174";
// DEX router addresses
pub const QUICKSWAP_ROUTER_ADDRESS_STR: &str = "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff";
pub const SUSHISWAP_ROUTER_ADDRESS_STR: &str = "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506";

// Simplified gas cost in USDC
pub const GAS_COST_USDC: f64 = 0.5;

// this is a helper function to parse addresses
pub fn parse_address(address_str: &str) -> Result<Address, anyhow::Error> {
    address_str.parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse address {}: {}", address_str, e))
}