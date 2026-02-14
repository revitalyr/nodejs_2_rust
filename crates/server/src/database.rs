use crate::error::AppError;
use ethereum_boilerplate_shared::types::{Address, U256};
use serde_json::Value;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use std::str::FromStr;
use tracing::info;

pub type DbPool = Pool<Postgres>;

pub struct DatabaseService {
    pool: DbPool,
}

impl DatabaseService {
    pub async fn new(database_url: &str) -> Result<Self, AppError> {
        info!("Connecting to database...");
        
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        info!("Database connected successfully");
        Ok(DatabaseService { pool })
    }

    pub async fn save_nft_metadata(
        &self,
        token_address: &str,
        token_id: &str,
        metadata: &Value,
    ) -> Result<(), AppError> {
        let token_address = Address::from_str(token_address)
            .map_err(|e| AppError::ParseError(format!("Invalid token address: {}", e)))?;
        let token_id = U256::from_str(token_id)
            .map_err(|e| AppError::ParseError(format!("Invalid token ID: {}", e)))?;

        let mut token_id_bytes = [0u8; 32];
        token_id_bytes.copy_from_slice(&token_id.to_be_bytes::<32>());
        
        sqlx::query(
            r#"
            INSERT INTO nft_metadata (token_address, token_id, metadata, created_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (token_address, token_id) 
            DO UPDATE SET metadata = $3, updated_at = NOW()
            "#
        )
        .bind(token_address.as_slice())
        .bind(&token_id_bytes[..])
        .bind(metadata)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_nft_metadata(
        &self,
        token_address: &str,
        token_id: &str,
    ) -> Result<Option<Value>, AppError> {
        let token_address = Address::from_str(token_address)
            .map_err(|e| AppError::ParseError(format!("Invalid token address: {}", e)))?;
        let token_id = U256::from_str(token_id)
            .map_err(|e| AppError::ParseError(format!("Invalid token ID: {}", e)))?;

        let mut token_id_bytes = [0u8; 32];
        token_id_bytes.copy_from_slice(&token_id.to_be_bytes::<32>());

        let result = sqlx::query(
            r#"
            SELECT metadata FROM nft_metadata 
            WHERE token_address = $1 AND token_id = $2
            "#
        )
        .bind(token_address.as_slice())
        .bind(&token_id_bytes[..])
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            let metadata: Value = row.get("metadata");
            Ok(Some(metadata))
        } else {
            Ok(None)
        }
    }

    pub async fn cache_wallet_balance(
        &self,
        address: &str,
        balance: &str,
        chain_id: u64,
    ) -> Result<(), AppError> {
        let address = Address::from_str(address)
            .map_err(|e| AppError::ParseError(format!("Invalid address: {}", e)))?;
        let balance = U256::from_str(balance)
            .map_err(|e| AppError::ParseError(format!("Invalid balance: {}", e)))?;

        let mut balance_bytes = [0u8; 32];
        balance_bytes.copy_from_slice(&balance.to_be_bytes::<32>());

        sqlx::query(
            r#"
            INSERT INTO wallet_balances (address, balance, chain_id, updated_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (address, chain_id) 
            DO UPDATE SET balance = $2, updated_at = NOW()
            "#
        )
        .bind(address.as_slice())
        .bind(&balance_bytes[..])
        .bind(chain_id as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_cached_wallet_balance(
        &self,
        address: &str,
        chain_id: u64,
    ) -> Result<Option<String>, AppError> {
        let address = Address::from_str(address)
            .map_err(|e| AppError::ParseError(format!("Invalid address: {}", e)))?;

        let result = sqlx::query(
            r#"
            SELECT balance FROM wallet_balances 
            WHERE address = $1 AND chain_id = $2 
            AND updated_at > NOW() - INTERVAL '5 minutes'
            "#
        )
        .bind(address.as_slice())
        .bind(chain_id as i64)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            let balance_bytes: &[u8] = row.get("balance");
            let balance = U256::from_be_bytes::<32>(balance_bytes.try_into().unwrap());
            Ok(Some(balance.to_string()))
        } else {
            Ok(None)
        }
    }

    pub async fn log_api_call(
        &self,
        endpoint: &str,
        address: &str,
        response_time_ms: u64,
    ) -> Result<(), AppError> {
        let address_bytes = if let Ok(addr) = alloy::primitives::Address::from_str(address) {
            Some(addr.as_slice().to_vec())
        } else {
            None
        };

        sqlx::query(
            r#"
            INSERT INTO api_logs (endpoint, address, response_time_ms, created_at)
            VALUES ($1, $2, $3, NOW())
            "#
        )
        .bind(endpoint)
        .bind(address_bytes)
        .bind(response_time_ms as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
