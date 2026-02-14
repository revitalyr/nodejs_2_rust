//! Shared utility functions used across the project

use crate::constants::*;
use crate::common_types::*;
use alloy::primitives::{Address, U256, TxHash as H256};
use std::time::Duration;

/// Convert seconds to Duration
pub fn secs_to_duration(seconds: u64) -> Duration {
    Duration::from_secs(seconds)
}

/// Convert milliseconds to Duration
pub fn ms_to_duration(milliseconds: u64) -> Duration {
    Duration::from_millis(milliseconds)
}

/// Get network info by chain ID
pub fn get_network_by_chain_id(chain_id: u64) -> Option<NetworkInfo> {
    SUPPORTED_NETWORKS.iter().find(|network| network.chain_id == chain_id).cloned()
}

/// Get network info by name
pub fn get_network_by_name(name: &str) -> Option<NetworkInfo> {
    SUPPORTED_NETWORKS.iter().find(|network| network.name.to_lowercase() == name.to_lowercase()).cloned()
}

/// Validate Ethereum address format
pub fn is_valid_address_format(address: &str) -> bool {
    address.len() >= validation::MIN_ADDRESS_LENGTH 
        && address.len() <= validation::MAX_ADDRESS_LENGTH
        && address.starts_with(validation::ADDRESS_PREFIX)
        && address[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Validate private key format
pub fn is_valid_private_key_format(private_key: &str) -> bool {
    private_key.len() >= validation::MIN_PRIVATE_KEY_LENGTH
        && private_key.len() <= validation::MAX_PRIVATE_KEY_LENGTH
        && private_key.starts_with(validation::PRIVATE_KEY_PREFIX)
        && private_key[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Format Ethereum address to checksum format
pub fn format_address_checksum(address: Address) -> String {
    format!("{:#x}", address)
}

/// Parse string to U256 amount with validation
pub fn parse_amount(amount_str: &str) -> Result<U256, AppError> {
    amount_str
        .parse::<U256>()
        .map_err(|_| AppError::ValidationError(format!("{}: {}", errors::INVALID_AMOUNT_PREFIX, amount_str)))
}

/// Convert wei to ether string representation
pub fn wei_to_ether(wei: U256) -> String {
    let ether_value = wei.to::<u128>() as f64 / 1e18;
    format!("{:.6}", ether_value)
}

/// Convert ether string to wei
pub fn ether_to_wei(ether_str: &str) -> Result<U256, AppError> {
    let ether_value: f64 = ether_str
        .parse()
        .map_err(|_| AppError::ValidationError(format!("{}: {}", errors::INVALID_AMOUNT_PREFIX, ether_str)))?;
    
    let wei_value = (ether_value * 1e18) as u128;
    Ok(U256::from(wei_value))
}

/// Calculate gas price based on strategy
pub fn calculate_gas_price(base_gas_price: U256, strategy: GasStrategy) -> U256 {
    let multiplier = strategy.multiplier();
    let base_value = base_gas_price.to::<u128>() as f64;
    let adjusted_value = base_value * multiplier;
    U256::from(adjusted_value as u128)
}

/// Create a timeout duration for network operations
pub fn network_timeout_duration() -> Duration {
    secs_to_duration(network::DEFAULT_TIMEOUT_SECS)
}

/// Create initial delay for exponential backoff
pub fn backoff_initial_delay() -> Duration {
    ms_to_duration(network::INITIAL_DELAY_MS)
}

/// Generate error message for invalid address
pub fn invalid_address_error(address: &str) -> AppError {
    AppError::ValidationError(format!("{}: {}", errors::INVALID_ADDRESS_PREFIX, address))
}

/// Generate error message for invalid private key
pub fn invalid_private_key_error(private_key: &str) -> AppError {
    AppError::ValidationError(format!("{}: {}", errors::INVALID_PRIVATE_KEY_PREFIX, private_key))
}

/// Generate error message for network timeout
pub fn network_timeout_error() -> AppError {
    AppError::NetworkError(errors::NETWORK_TIMEOUT.to_string())
}

/// Generate error message for max retries exceeded
pub fn max_retries_exceeded_error(details: &str) -> AppError {
    AppError::NetworkError(format!("{}: {}", errors::MAX_RETRIES_EXCEEDED, details))
}

/// Generate error message for invalid RPC URL
pub fn invalid_rpc_url_error(url: &str) -> AppError {
    AppError::ConfigurationError(format!("{}: {}", errors::INVALID_RPC_URL_PREFIX, url))
}

/// Check if an address is a zero address
pub fn is_zero_address(address: Address) -> bool {
    address == Address::default()
}

/// Create a zero address
pub fn zero_address() -> Address {
    Address::default()
}

/// Create a zero hash
pub fn zero_hash() -> H256 {
    H256::default()
}

/// Validate that an address is not zero
pub fn validate_nonzero_address(address: Address) -> Result<Address, AppError> {
    if is_zero_address(address) {
        Err(invalid_address_error("zero address"))
    } else {
        Ok(address)
    }
}

/// Format transaction hash for display
pub fn format_transaction_hash(hash: H256) -> String {
    format!("{:#x}", hash)
}

/// Truncate address for display (show first 6 and last 4 characters)
pub fn truncate_address(address: Address) -> String {
    let address_str = format!("{:#x}", address);
    if address_str.len() <= 10 {
        address_str
    } else {
        format!("{}...{}", &address_str[..6], &address_str[address_str.len()-4..])
    }
}

/// Create a standard contract interaction result
pub fn create_contract_result(
    transaction_hash: H256,
    status: TransactionStatus,
) -> ContractInteractionResult {
    ContractInteractionResult {
        transaction_hash,
        status,
        gas_used: None,
        block_number: None,
        error_message: None,
    }
}

/// Create a failed contract interaction result
pub fn create_contract_error(
    transaction_hash: Option<H256>,
    error_message: String,
) -> ContractInteractionResult {
    ContractInteractionResult {
        transaction_hash: transaction_hash.unwrap_or_else(zero_hash),
        status: TransactionStatus::Failed,
        gas_used: None,
        block_number: None,
        error_message: Some(error_message),
    }
}
