use serde::{Deserialize, Serialize};

// Re-export commonly used alloy types
pub use alloy::primitives::{Address, TxHash as H256, U256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTBalance {
    pub token_address: Address,
    pub token_id: U256,
    pub token_uri: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERC20Balance {
    pub token_address: Address,
    pub balance: U256,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: H256,
    pub from: Address,
    pub to: Option<Address>,
    pub value: U256,
    pub gas_used: U256,
    pub gas_price: Option<U256>,
    pub block_number: u64,
    pub block_hash: H256,
    pub transaction_index: u64,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTTransfer {
    pub token_address: Address,
    pub from: Address,
    pub to: Address,
    pub token_id: U256,
    pub transaction_hash: H256,
    pub block_number: u64,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERC20Transfer {
    pub token_address: Address,
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub transaction_hash: H256,
    pub block_number: u64,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: Address,
    pub balance: U256,
    pub nonce: U256,
}
