use crate::types::{ArbitrageOpportunity, DexPrice};
use crate::constants::GAS_COST_USDC;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn find_arbitrage_opportunities(
    prices: &[DexPrice],
    trade_amount: f64,
    min_profit_threshold: f64,
) -> Vec<ArbitrageOpportunity> {
    let mut opportunities = Vec::new();
    
    if prices.len() < 2 {
        return opportunities; // need at least two prices to compare
    }
    
    // compare each pair of DEX prices
    for i in 0..prices.len() {
        for j in i + 1..prices.len() {
            let price_a = &prices[i];
            let price_b = &prices[j];
            
            // Check both directions
            if let Some(opportunity) = check_arbitrage_pair(price_a, price_b, trade_amount, min_profit_threshold) {
                opportunities.push(opportunity);
            }
            
            if let Some(opportunity) = check_arbitrage_pair(price_b, price_a, trade_amount, min_profit_threshold) {
                opportunities.push(opportunity);
            }
        }
    }
    
    opportunities
}

fn check_arbitrage_pair(
    buy_dex: &DexPrice,
    sell_dex: &DexPrice,
    trade_amount: f64,
    min_profit_threshold: f64,
) -> Option<ArbitrageOpportunity> {

    // if buy price is lower than sell price then only there's an opportunity
    if buy_dex.price < sell_dex.price {
        let revenue = trade_amount * sell_dex.price; 
        let cost = trade_amount * buy_dex.price;     
        let gross_profit = revenue - cost;
        let net_profit = gross_profit - GAS_COST_USDC;
        
        // check if opportunities are above the threshold or not
        if net_profit > min_profit_threshold {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            return Some(ArbitrageOpportunity {
                buy_dex: buy_dex.dex_name.clone(),
                sell_dex: sell_dex.dex_name.clone(),
                buy_price: buy_dex.price,
                sell_price: sell_dex.price,
                profit: net_profit,
                timestamp,
            });
        }
    }
    
    None
}