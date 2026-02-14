use alloy::primitives::{Address, U256};
use alloy::primitives::aliases::B256;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Module for centralized management of large number formats.
/// Ethereum Wallet and API typically expect strings to avoid Floating Point issues in JS.
pub mod u256_ser {
    use super::*;
    use serde::{Serializer, Deserializer};
    use std::str::FromStr;

    pub fn serialize<S: Serializer>(value: &U256, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<U256, D::Error> {
        let s = String::deserialize(deserializer)?;
        U256::from_str(&s).map_err(serde::de::Error::custom)
    }
}

// --- Balance Models ---------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTBalance {
    pub token_address: Address,
    #[serde(with = "u256_ser")]
    pub token_id: U256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERC20Balance {
    pub token_address: Address,
    #[serde(with = "u256_ser")]
    pub balance: U256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    pub decimals: u8,
}

// --- Transactions and Transfers ---------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: B256,
    pub from: Address,
    pub to: Option<Address>,
    #[serde(with = "u256_ser")]
    pub value: U256,
    #[serde(with = "u256_ser")]
    pub gas_used: U256,
    // Use built-in serde Option handling with custom module
    #[serde(default, with = "u256_ser_option", skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,
    pub block_number: u64,
    pub block_hash: B256,
    pub transaction_index: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    pub status: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTTransfer {
    pub token_address: Address,
    pub from: Address,
    pub to: Address,
    #[serde(with = "u256_ser")]
    pub token_id: U256,
    pub transaction_hash: B256,
    pub block_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERC20Transfer {
    pub token_address: Address,
    pub from: Address,
    pub to: Address,
    #[serde(with = "u256_ser")]
    pub value: U256,
    pub transaction_hash: B256,
    pub block_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

// --- Wallet Information --------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: Address,
    #[serde(with = "u256_ser")]
    pub balance: U256,
    #[serde(with = "u256_ser")]
    pub nonce: U256,
}

// --- Query Parameters (DTO) -----------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddressQuery {
    pub address: Address,
}

/// Helper module for Option<U256>
mod u256_ser_option {
    use super::*;
    use serde::{Serializer, Deserializer};

    pub fn serialize<S: Serializer>(opt: &Option<U256>, s: S) -> Result<S::Ok, S::Error> {
        match opt {
            Some(v) => u256_ser::serialize(v, s),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<U256>, D::Error> {
        let opt = Option::<String>::deserialize(d)?;
        match opt {
            Some(s) => Ok(Some(std::str::FromStr::from_str(&s).map_err(serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}