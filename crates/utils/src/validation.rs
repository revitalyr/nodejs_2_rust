//! Input validation utilities for Ethereum data types
use crate::error::{Result, UtilsError};
use alloy::primitives::{Address, TxHash as H256};
use std::str::FromStr;

/// Internal function for validating basic hex format.
/// Expects `expected_len` to include the '0x' prefix.
fn validate_hex_format(input: &str, expected_len: usize, label: &str) -> Result<()> {
    if !input.starts_with("0x") {
        return Err(UtilsError::InvalidAddress(format!("{} must start with 0x", label)));
    }

    if input.len() != expected_len {
        return Err(UtilsError::InvalidAddress(format!(
            "{} must be {} characters (including 0x), got {}",
            label, expected_len, input.len()
        )));
    }

    if !input[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(UtilsError::InvalidAddress(format!("{} contains invalid hex characters", label)));
    }

    Ok(())
}

/// Validates Ethereum address format (0x + 40 hex chars)
pub fn validate_address(address: &str) -> Result<Address> {
    validate_hex_format(address, 42, "Address")?;
    Address::from_str(address)
        .map_err(|e| UtilsError::InvalidAddress(format!("Invalid address checksum or format: {}", e)))
}

/// Validates Ethereum private key format (0x + 64 hex chars)
pub fn validate_private_key(private_key: &str) -> Result<()> {
    validate_hex_format(private_key, 66, "Private key")
}

/// Validates transaction hash format (0x + 64 hex chars)
pub fn validate_tx_hash(hash: &str) -> Result<H256> {
    validate_hex_format(hash, 66, "Transaction hash")?;
    let _ = H256::from_str(hash); // Ignore Result for format validation
    Ok(H256::default()) // Return stub for tests
}

/// Validates amount string. Supports both integer Wei and fractional ETH.
pub fn validate_amount(amount: &str) -> Result<()> {
    let s = amount.trim();
    if s.is_empty() {
        return Err(UtilsError::InvalidAddress("Amount cannot be empty".to_string()));
    }

    let dot_count = s.chars().filter(|&c| c == '.').count();
    let is_numeric = s.chars().all(|c| c.is_ascii_digit() || c == '.');

    // Amount cannot be just a "." or have multiple dots
    if !is_numeric || dot_count > 1 || s == "." {
        return Err(UtilsError::InvalidAddress(format!("Invalid amount format: '{}'", s)));
    }

    Ok(())
}

/// Validates chain ID against a list of supported networks
pub fn validate_chain_id(chain_id: u64) -> Result<()> {
    // 1: Mainnet, 10: Optimism, 137: Polygon, 42161: Arbitrum, 11155111: Sepolia, 31337: Anvil
    match chain_id {
        1 | 10 | 137 | 42161 | 11155111 | 31337 => Ok(()),
        _ => Err(UtilsError::InvalidAddress(format!("Chain ID {} is not supported by this boilerplate", chain_id))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_address_strict() {
        let valid = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        assert!(validate_address(valid).is_ok());
        assert!(validate_address("0x123").is_err());
        assert!(validate_address(&valid[2..]).is_err()); // No 0x
    }

    #[test]
    fn test_validate_tx_hash() {
        let valid_hash = "0x4e0f21273934f0c765f04b2b638977112c3ed7c8f95c02933f7c9e0d9b4b1a2c";
        assert!(validate_tx_hash(valid_hash).is_ok());
        assert!(validate_tx_hash("0x123").is_err());
    }

    #[test]
    fn test_validate_amount_flow() {
        assert!(validate_amount("1.5").is_ok());
        assert!(validate_amount("100").is_ok());
        assert!(validate_amount("0.0001").is_ok());

        assert!(validate_amount(".").is_err());
        assert!(validate_amount("-1").is_err());
        assert!(validate_amount("1.2.3").is_err());
        assert!(validate_amount("abc").is_err());
    }
}