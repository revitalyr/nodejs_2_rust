//! Smart contract utilities and management for Ethereum Boilerplate
//! 
//! This crate provides:
//! - Contract ABI generation using Alloy's sol! macro
//! - Contract deployment and interaction utilities
//! - Network configuration management
//! - Type-safe contract interfaces

use alloy::sol;
use alloy::primitives::Address;
use std::collections::HashMap;
use ethereum_boilerplate_shared::SUPPORTED_NETWORKS;

// Generate typed contracts using Alloy's sol! macro
sol! {
    interface IERC20 {
        function balanceOf(address account) external view returns (uint256);
        function transfer(address to, uint256 amount) external returns (bool);
        function mint(address to, uint256 amount) external;
    }
    
    interface IERC721 {
        function balanceOf(address owner) external view returns (uint256);
        function safeMint(address to, string calldata uri) external;
        function safeTransferFrom(address from, address to, uint256 tokenId) external;
    }
}

/// Contract manager for tracking deployed contracts
#[derive(Debug, Clone)]
pub struct ContractManager {
    contracts: HashMap<String, Address>,
}

impl ContractManager {
    /// Create new contract manager
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
        }
    }

    /// Add contract address manually
    pub fn add_contract(&mut self, name: String, address: Address) {
        self.contracts.insert(name, address);
    }

    /// Get deployed contract address by name
    pub fn get_contract(&self, name: &str) -> Option<Address> {
        self.contracts.get(name).copied()
    }

    /// List all deployed contracts
    pub fn list_contracts(&self) -> Vec<(String, Address)> {
        self.contracts
            .iter()
            .map(|(name, addr)| (name.clone(), *addr))
            .collect()
    }

    /// Remove contract from tracking
    pub fn remove_contract(&mut self, name: &str) -> Option<Address> {
        self.contracts.remove(name)
    }
}

/// Network configuration for supported chains
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub name: String,
    pub chain_id: u64,
    pub rpc_url: String,
}

/// Get all supported network configurations
pub fn get_supported_networks() -> Vec<NetworkConfig> {
    SUPPORTED_NETWORKS
        .iter()
        .map(|network| NetworkConfig {
            name: network.name.clone(),
            chain_id: network.chain_id,
            rpc_url: network.rpc_url.clone(),
        })
        .collect()
}