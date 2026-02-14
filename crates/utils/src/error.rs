//! Common error types and utilities

use thiserror::Error;

pub type Result<T> = std::result::Result<T, UtilsError>;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Ethereum error: {0}")]
    Ethereum(String),
    
    #[error("Signer error: {0}")]
    Signer(#[from] alloy::signers::local::LocalSignerError),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    #[error("Network not supported: {0}")]
    UnsupportedNetwork(String),
    
    #[error("Contract error: {0}")]
    Contract(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Interactive error: {0}")]
    Interactive(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl UtilsError {
    pub fn config_error(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }
    
    pub fn validation_error(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
    
    pub fn parse(msg: impl Into<String>) -> Self {
        Self::Parse(msg.into())
    }
    
    pub fn invalid_address(address: impl Into<String>) -> Self {
        Self::InvalidAddress(address.into())
    }
    
    pub fn invalid_private_key(key: impl Into<String>) -> Self {
        Self::InvalidPrivateKey(key.into())
    }
    
    pub fn invalid_amount(amount: impl Into<String>) -> Self {
        Self::InvalidAmount(amount.into())
    }
    
    pub fn unsupported_network(network: impl Into<String>) -> Self {
        Self::UnsupportedNetwork(network.into())
    }
    
    pub fn contract(msg: impl Into<String>) -> Self {
        Self::Contract(msg.into())
    }
    
    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }
    
    pub fn network(error: reqwest::Error) -> Self {
        Self::Network(error)
    }
    
    pub fn interactive_error(msg: impl Into<String>) -> Self {
        Self::Interactive(msg.into())
    }
    
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

// Automatic conversion from anyhow::Error
impl From<anyhow::Error> for UtilsError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err.to_string())
    }
}
