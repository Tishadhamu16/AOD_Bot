
# Polygon Arbitrage Opportunity Detector Bot

A Rust-based application that detects arbitrage opportunities between DEXes on the Polygon network.

## Features

- Real-time price monitoring of WETH/USDC pairs
- Multi-DEX support
- Arbitrage opportunity detection with configurable thresholds, polling intervals and trade parameters
- Simulated profit calculation including gas costs
- PostgreSQL database logging 

## Technology Stack

- **Blockchain**: Polygon Network
- **Programming Language**: Rust
- **DEX Protocols**: Uniswap V2 (QuickSwap, SushiSwap)
- **Database**: PostgreSQL
- **Key Libraries**: 
  - `ethers-rs` for blockchain interaction
  - `sqlx` for database operations
  - `tokio` for async runtime
  - `env_logger` for logging

## Configuration

Create a `.env` file:

POLYGON_RPC_URL=https://polygon-rpc.com
MIN_PROFIT_THRESHOLD=10.0
TRADE_AMOUNT=1.0
POLL_INTERVAL_MS=5000
DATABASE_URL=postgres://your_username:your_password@localhost:5432/arbitrage_bot
DEDUPLICATION_WINDOW_SECS=300

