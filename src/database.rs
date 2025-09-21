use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;
use crate::types::ArbitrageOpportunity;
use chrono::{DateTime, Utc};

pub struct Database {
    pool: PgPool,
}

impl Database {
    // Initializing database connection
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        // Creating table 
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS arbitrage_opportunities (
                id SERIAL PRIMARY KEY,
                buy_dex VARCHAR NOT NULL,
                sell_dex VARCHAR NOT NULL,
                buy_price DOUBLE PRECISION NOT NULL,
                sell_price DOUBLE PRECISION NOT NULL,
                profit DOUBLE PRECISION NOT NULL,
                timestamp BIGINT NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&pool)
        .await?;
        
        log::info!("Connected to database successfully");
        Ok(Self { pool })
    }
    
    // Store an arbitrage opportunity in the database
    pub async fn store_opportunity(&self, opportunity: &ArbitrageOpportunity) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO arbitrage_opportunities 
            (buy_dex, sell_dex, buy_price, sell_price, profit, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(&opportunity.buy_dex)
        .bind(&opportunity.sell_dex)
        .bind(opportunity.buy_price)
        .bind(opportunity.sell_price)
        .bind(opportunity.profit)
        .bind(opportunity.timestamp as i64)
        .execute(&self.pool)
        .await?;
        
        log::debug!("Stored arbitrage opportunity in database");
        Ok(())
    }

}