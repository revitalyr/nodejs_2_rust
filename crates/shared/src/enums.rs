//! Shared enums for the Ethereum Boilerplate project

use serde::{Deserialize, Serialize};
use std::fmt::{Display, self};

/// Contract action types for interactions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractAction {
    Balance,
    Mint,
    Transfer,
    Approve,
    MintNft,
    TransferNft,
}

impl Display for ContractAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractAction::Balance => write!(f, "balance"),
            ContractAction::Mint => write!(f, "mint"),
            ContractAction::Transfer => write!(f, "transfer"),
            ContractAction::Approve => write!(f, "approve"),
            ContractAction::MintNft => write!(f, "mint-nft"),
            ContractAction::TransferNft => write!(f, "transfer-nft"),
        }
    }
}

impl ContractAction {
    /// Get all possible contract actions
    pub fn all() -> &'static [ContractAction] {
        &[
            ContractAction::Balance,
            ContractAction::Mint,
            ContractAction::Transfer,
            ContractAction::Approve,
            ContractAction::MintNft,
            ContractAction::TransferNft,
        ]
    }

    /// Get action description for CLI help
    pub fn description(&self) -> &'static str {
        match self {
            ContractAction::Balance => "Get token balance",
            ContractAction::Mint => "Mint new tokens",
            ContractAction::Transfer => "Transfer tokens to another address",
            ContractAction::Approve => "Approve tokens for spending",
            ContractAction::MintNft => "Mint new NFT",
            ContractAction::TransferNft => "Transfer NFT to another address",
        }
    }

    /// Parse string to ContractAction
    pub fn parse(s: &str) -> Option<ContractAction> {
        match s.to_lowercase().as_str() {
            "balance" => Some(ContractAction::Balance),
            "mint" => Some(ContractAction::Mint),
            "transfer" => Some(ContractAction::Transfer),
            "approve" => Some(ContractAction::Approve),
            "mint-nft" => Some(ContractAction::MintNft),
            "transfer-nft" => Some(ContractAction::TransferNft),
            _ => None,
        }
    }
}

/// Contract template types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractTemplate {
    Erc20,
    Erc721,
    Custom,
}

impl Display for ContractTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractTemplate::Erc20 => write!(f, "erc20"),
            ContractTemplate::Erc721 => write!(f, "erc721"),
            ContractTemplate::Custom => write!(f, "custom"),
        }
    }
}

impl ContractTemplate {
    /// Get all possible contract templates
    pub fn all() -> &'static [ContractTemplate] {
        &[ContractTemplate::Erc20, ContractTemplate::Erc721, ContractTemplate::Custom]
    }

    /// Get template description
    pub fn description(&self) -> &'static str {
        match self {
            ContractTemplate::Erc20 => "Standard ERC20 token contract",
            ContractTemplate::Erc721 => "Standard ERC721 NFT contract",
            ContractTemplate::Custom => "Custom contract template",
        }
    }

    /// Get default contract name for template
    pub fn default_name(&self) -> &'static str {
        match self {
            ContractTemplate::Erc20 => "MyToken",
            ContractTemplate::Erc721 => "MyNFT",
            ContractTemplate::Custom => "CustomContract",
        }
    }

    /// Parse string to ContractTemplate
    pub fn parse(s: &str) -> Option<ContractTemplate> {
        match s.to_lowercase().as_str() {
            "erc20" => Some(ContractTemplate::Erc20),
            "erc721" => Some(ContractTemplate::Erc721),
            "custom" => Some(ContractTemplate::Custom),
            _ => None,
        }
    }
}

/// Contract deployment types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeploymentType {
    Single,
    All,
}

impl Display for DeploymentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeploymentType::Single => write!(f, "single"),
            DeploymentType::All => write!(f, "all"),
        }
    }
}

impl DeploymentType {
    /// Parse string to DeploymentType
    pub fn parse(s: &str) -> Option<DeploymentType> {
        match s.to_lowercase().as_str() {
            "single" => Some(DeploymentType::Single),
            "all" => Some(DeploymentType::All),
            _ => None,
        }
    }
}

/// Transaction status types (enhanced version)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TxStatus {
    Pending,
    Confirmed,
    Failed,
    Replaced,
    Dropped,
}

impl Display for TxStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TxStatus::Pending => write!(f, "pending"),
            TxStatus::Confirmed => write!(f, "confirmed"),
            TxStatus::Failed => write!(f, "failed"),
            TxStatus::Replaced => write!(f, "replaced"),
            TxStatus::Dropped => write!(f, "dropped"),
        }
    }
}

/// Gas price strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GasPriceStrategy {
    Slow,
    Standard,
    Fast,
    Urgent,
}

impl Display for GasPriceStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GasPriceStrategy::Slow => write!(f, "slow"),
            GasPriceStrategy::Standard => write!(f, "standard"),
            GasPriceStrategy::Fast => write!(f, "fast"),
            GasPriceStrategy::Urgent => write!(f, "urgent"),
        }
    }
}

impl GasPriceStrategy {
    /// Get gas price multiplier
    pub fn multiplier(&self) -> f64 {
        match self {
            GasPriceStrategy::Slow => 0.8,
            GasPriceStrategy::Standard => 1.0,
            GasPriceStrategy::Fast => 1.2,
            GasPriceStrategy::Urgent => 1.5,
        }
    }

    /// Parse string to GasPriceStrategy
    pub fn parse(s: &str) -> Option<GasPriceStrategy> {
        match s.to_lowercase().as_str() {
            "slow" => Some(GasPriceStrategy::Slow),
            "standard" => Some(GasPriceStrategy::Standard),
            "fast" => Some(GasPriceStrategy::Fast),
            "urgent" => Some(GasPriceStrategy::Urgent),
            _ => None,
        }
    }
}

/// Network environment types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NetworkEnvironment {
    Mainnet,
    Testnet,
    Local,
}

impl Display for NetworkEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkEnvironment::Mainnet => write!(f, "mainnet"),
            NetworkEnvironment::Testnet => write!(f, "testnet"),
            NetworkEnvironment::Local => write!(f, "local"),
        }
    }
}

impl NetworkEnvironment {
    /// Parse string to NetworkEnvironment
    pub fn parse(s: &str) -> Option<NetworkEnvironment> {
        match s.to_lowercase().as_str() {
            "mainnet" => Some(NetworkEnvironment::Mainnet),
            "testnet" => Some(NetworkEnvironment::Testnet),
            "local" => Some(NetworkEnvironment::Local),
            _ => None,
        }
    }
}
