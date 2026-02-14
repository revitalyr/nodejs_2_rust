//! Shared Ethereum types and common structures

use alloy::primitives::{Address, U256, TxHash as H256};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// Common network information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub name: String,
    pub chain_id: u64,
    pub rpc_url: String,
    pub block_explorer_url: Option<String>,
    pub native_currency: CurrencyInfo,
}

/// Currency information for networks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

/// Supported networks configuration
pub static SUPPORTED_NETWORKS: LazyLock<Vec<NetworkInfo>> = LazyLock::new(|| {
    vec![
        NetworkInfo {
            name: "Ethereum Mainnet".to_string(),
            chain_id: 1,
            rpc_url: "https://eth-mainnet.g.alchemy.com/v2/your-api-key".to_string(),
            block_explorer_url: Some("https://etherscan.io".to_string()),
            native_currency: CurrencyInfo {
                name: "Ethereum".to_string(),
                symbol: "ETH".to_string(),
                decimals: 18,
            },
        },
        NetworkInfo {
            name: "Sepolia Testnet".to_string(),
            chain_id: 11155111,
            rpc_url: "https://eth-sepolia.g.alchemy.com/v2/your-api-key".to_string(),
            block_explorer_url: Some("https://sepolia.etherscan.io".to_string()),
            native_currency: CurrencyInfo {
                name: "Ethereum".to_string(),
                symbol: "ETH".to_string(),
                decimals: 18,
            },
        },
        NetworkInfo {
            name: "Polygon Mainnet".to_string(),
            chain_id: 137,
            rpc_url: "https://polygon-mainnet.g.alchemy.com/v2/your-api-key".to_string(),
            block_explorer_url: Some("https://polygonscan.com".to_string()),
            native_currency: CurrencyInfo {
                name: "Matic".to_string(),
                symbol: "MATIC".to_string(),
                decimals: 18,
            },
        },
        NetworkInfo {
            name: "Mumbai Testnet".to_string(),
            chain_id: 80001,
            rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/your-api-key".to_string(),
            block_explorer_url: Some("https://mumbai.polygonscan.com".to_string()),
            native_currency: CurrencyInfo {
                name: "Matic".to_string(),
                symbol: "MATIC".to_string(),
                decimals: 18,
            },
        },
    ]
});

/// Transaction status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Replaced,
}

/// Contract interaction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInteractionResult {
    pub transaction_hash: H256,
    pub status: TransactionStatus,
    pub gas_used: Option<U256>,
    pub block_number: Option<u64>,
    pub error_message: Option<String>,
}

/// Token information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Option<U256>,
}

/// NFT metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub attributes: Option<Vec<NFTAttribute>>,
}

/// NFT attribute structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTAttribute {
    pub trait_type: String,
    pub value: serde_json::Value,
}

/// Wallet information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: Address,
    pub private_key: Option<String>,
    pub network_name: String,
    pub balance: Option<U256>,
    pub nonce: Option<U256>,
}

/// Common error types for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    NetworkError(String),
    ValidationError(String),
    ContractError(String),
    TransactionError(String),
    ConfigurationError(String),
    InternalError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::ContractError(msg) => write!(f, "Contract error: {}", msg),
            AppError::TransactionError(msg) => write!(f, "Transaction error: {}", msg),
            AppError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// Result type alias for the application
pub type AppResult<T> = Result<T, AppError>;

/// Common contract ABI definitions
pub mod contract_abis {
    // Standard ERC20 ABI events and functions
    pub const ERC20_TRANSFER_EVENT: &str = "event Transfer(address indexed from, address indexed to, uint256 value)";
    pub const ERC20_APPROVAL_EVENT: &str = "event Approval(address indexed owner, address indexed spender, uint256 value)";
    pub const ERC20_BALANCE_FUNCTION: &str = "function balanceOf(address account) view returns (uint256)";
    pub const ERC20_TRANSFER_FUNCTION: &str = "function transfer(address to, uint256 amount) returns (bool)";
    pub const ERC20_APPROVE_FUNCTION: &str = "function approve(address spender, uint256 amount) returns (bool)";
    
    // Standard ERC721 ABI events and functions
    pub const ERC721_TRANSFER_EVENT: &str = "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)";
    pub const ERC721_APPROVAL_EVENT: &str = "event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId)";
    pub const ERC721_OWNER_FUNCTION: &str = "function ownerOf(uint256 tokenId) view returns (address)";
    pub const ERC721_BALANCE_FUNCTION: &str = "function balanceOf(address owner) view returns (uint256)";
    pub const ERC721_TRANSFER_FUNCTION: &str = "function transferFrom(address from, address to, uint256 tokenId)";
    pub const ERC721_APPROVE_FUNCTION: &str = "function approve(address to, uint256 tokenId)";
}

/// Gas price estimation strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GasStrategy {
    Slow,
    Standard,
    Fast,
    Urgent,
}

impl GasStrategy {
    /// Get gas price multiplier for the strategy
    pub fn multiplier(self) -> f64 {
        match self {
            GasStrategy::Slow => 0.8,
            GasStrategy::Standard => 1.0,
            GasStrategy::Fast => 1.2,
            GasStrategy::Urgent => 1.5,
        }
    }
}
