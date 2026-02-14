//! Network utilities and helpers for Ethereum interactions

use alloy::providers::{Provider, ReqwestProvider};
use alloy::primitives::{Address, U256};
use std::sync::Arc;
use crate::error::{Result, UtilsError};
use crate::config::NetworkConfig;

/// Helper for converting provider errors to UtilsError::Ethereum format
fn map_eth_err<E: std::fmt::Display>(e: E) -> UtilsError {
    UtilsError::Ethereum(e.to_string())
}

/// Creates Ethereum provider
pub fn create_provider(network: &NetworkConfig) -> Result<Arc<ReqwestProvider>> {
    let url = network.rpc_url.parse().map_err(|e| 
        UtilsError::config_error(format!("Invalid RPC URL: {}", e))
    )?;
    
    let provider = ReqwestProvider::new_http(url);
    Ok(Arc::new(provider))
}

/// Gets chain ID
pub async fn get_chain_id(provider: &ReqwestProvider) -> Result<u64> {
    let chain_id = provider.get_chain_id().await.map_err(map_eth_err)?;
    Ok(chain_id)
}

/// Gets latest block number
pub async fn get_latest_block_number(provider: &ReqwestProvider) -> Result<u64> {
    let block_number = provider.get_block_number().await.map_err(map_eth_err)?;
    Ok(block_number)
}

/// Gets gas price
pub async fn get_gas_price(provider: &ReqwestProvider) -> Result<U256> {
    let gas_price = provider.get_gas_price().await.map_err(map_eth_err)?;
    Ok(U256::from(gas_price)) // Явное преобразование u128 → U256
}

/// Check provider health
pub async fn check_provider_health(provider: &ReqwestProvider) -> Result<()> {
    let _ = provider.get_block_number().await.map_err(map_eth_err)?;
    Ok(())
}

/// Get contract creation block 
pub async fn get_contract_creation_block(
    provider: &ReqwestProvider,
    address: Address,
) -> Result<u64> {
    let code = provider.get_code_at(address).await.map_err(map_eth_err)?;
    
    if !code.is_empty() {
        let latest_block = provider.get_block_number().await.map_err(map_eth_err)?;
        Ok(latest_block)
    } else {
        Ok(0)
    }
}