//! Configuration management utilities

use serde::{Deserialize, Serialize};
use crate::error::{Result, UtilsError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ethereum_rpc_url: String,
    pub moralis_api_key: Option<String>,
    pub database_url: Option<String>,
    pub log_level: String,
    pub server_port: u16,
    pub network: NetworkConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub name: String,
    pub chain_id: u64,
    pub rpc_url: String,
    pub explorer_url: String,
    pub native_currency: String,
    pub block_time: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ethereum_rpc_url: "http://localhost:8545".to_string(),
            moralis_api_key: None,
            database_url: None,
            log_level: "info".to_string(),
            server_port: 3000,
            network: NetworkConfig::localhost(),
        }
    }
}

impl NetworkConfig {
    pub fn localhost() -> Self {
        Self {
            name: "Localhost".to_string(),
            chain_id: 31337,
            rpc_url: "http://localhost:8545".to_string(),
            explorer_url: "".to_string(),
            native_currency: "ETH".to_string(),
            block_time: 2,
        }
    }
    
    pub fn mainnet() -> Self {
        Self {
            name: "Ethereum Mainnet".to_string(),
            chain_id: 1,
            rpc_url: "https://mainnet.infura.io/v3/YOUR_PROJECT_ID".to_string(),
            explorer_url: "https://etherscan.io".to_string(),
            native_currency: "ETH".to_string(),
            block_time: 12,
        }
    }
    
    pub fn sepolia() -> Self {
        Self {
            name: "Ethereum Sepolia".to_string(),
            chain_id: 11155111,
            rpc_url: "https://sepolia.infura.io/v3/YOUR_PROJECT_ID".to_string(),
            explorer_url: "https://sepolia.etherscan.io".to_string(),
            native_currency: "ETH".to_string(),
            block_time: 12,
        }
    }
    
    pub fn polygon() -> Self {
        Self {
            name: "Polygon".to_string(),
            chain_id: 137,
            rpc_url: "https://polygon-mainnet.infura.io/v3/YOUR_PROJECT_ID".to_string(),
            explorer_url: "https://polygonscan.com".to_string(),
            native_currency: "MATIC".to_string(),
            block_time: 2,
        }
    }
    
    pub fn arbitrum() -> Self {
        Self {
            name: "Arbitrum One".to_string(),
            chain_id: 42161,
            rpc_url: "https://arbitrum-mainnet.infura.io/v3/YOUR_PROJECT_ID".to_string(),
            explorer_url: "https://arbiscan.io".to_string(),
            native_currency: "ETH".to_string(),
            block_time: 1,
        }
    }
    
    pub fn optimism() -> Self {
        Self {
            name: "Optimism".to_string(),
            chain_id: 10,
            rpc_url: "https://optimism-mainnet.infura.io/v3/YOUR_PROJECT_ID".to_string(),
            explorer_url: "https://optimistic.etherscan.io".to_string(),
            native_currency: "ETH".to_string(),
            block_time: 2,
        }
    }
    
    pub fn get_by_chain_id(chain_id: u64) -> Option<Self> {
        match chain_id {
            1 => Some(Self::mainnet()),
            11155111 => Some(Self::sepolia()),
            137 => Some(Self::polygon()),
            42161 => Some(Self::arbitrum()),
            10 => Some(Self::optimism()),
            31337 => Some(Self::localhost()),
            _ => None,
        }
    }
    
    pub fn get_by_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "mainnet" | "ethereum" => Some(Self::mainnet()),
            "sepolia" => Some(Self::sepolia()),
            "polygon" | "matic" => Some(Self::polygon()),
            "arbitrum" | "arb" => Some(Self::arbitrum()),
            "optimism" | "op" => Some(Self::optimism()),
            "localhost" | "local" => Some(Self::localhost()),
            _ => None,
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        // dotenv::dotenv().ok(); // Commented out for now
        
        Ok(Self {
            ethereum_rpc_url: std::env::var("ETHEREUM_RPC_URL")
                .unwrap_or_else(|_| "http://localhost:8545".to_string()),
            moralis_api_key: std::env::var("MORALIS_API_KEY").ok(),
            database_url: std::env::var("DATABASE_URL").ok(),
            log_level: std::env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            network: NetworkConfig::localhost(),
        })
    }
    
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| UtilsError::config_error(format!("Failed to read config file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| UtilsError::config_error(format!("Failed to parse config file: {}", e)))
    }
    
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| UtilsError::config_error(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| UtilsError::config_error(format!("Failed to write config file: {}", e)))
    }
    
    pub fn validate(&self) -> Result<()> {
        if self.ethereum_rpc_url.is_empty() {
            return Err(UtilsError::config_error("ETHEREUM_RPC_URL is required"));
        }
        
        if self.server_port == 0 {
            return Err(UtilsError::config_error("SERVER_PORT must be > 0"));
        }
        
        Ok(())
    }
    
    pub fn set_network(&mut self, network_name: &str) -> Result<()> {
        self.network = NetworkConfig::get_by_name(network_name)
            .ok_or_else(|| UtilsError::config_error(format!("Unknown network: {}", network_name)))?;
        Ok(())
    }
}
