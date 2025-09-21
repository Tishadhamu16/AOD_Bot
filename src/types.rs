use ethers::types::Address;

#[derive(Debug, Clone)]
pub struct DexPrice {
    pub dex_name: String,
    pub price: f64,
    pub router_address: Address,
}

#[derive(Debug)]
pub struct ArbitrageOpportunity {
    pub buy_dex: String,
    pub sell_dex: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub profit: f64,
    pub timestamp: u64,
}