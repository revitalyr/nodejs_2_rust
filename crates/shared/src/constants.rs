//! Shared constants used across the Ethereum Boilerplate project

/// Network-related constants
pub mod network {
    /// Default timeout for network operations in seconds
    pub const DEFAULT_TIMEOUT_SECS: u64 = 10;
    
    /// Maximum number of retries for network operations
    pub const MAX_RETRIES: u32 = 3;
    
    /// Initial delay for exponential backoff in milliseconds
    pub const INITIAL_DELAY_MS: u64 = 1000;
    
    /// Gas limit for contract deployments
    pub const DEPLOYMENT_GAS_LIMIT: u64 = 8_000_000;
    
    /// Gas limit for contract interactions
    pub const INTERACTION_GAS_LIMIT: u64 = 200_000;
}

/// Ethereum-related constants
pub mod ethereum {
    /// Ethereum address length in bytes
    pub const ADDRESS_LENGTH: usize = 20;
    
    /// Private key length in bytes
    pub const PRIVATE_KEY_LENGTH: usize = 32;
    
    /// Chain ID for Ethereum Mainnet
    pub const MAINNET_CHAIN_ID: u64 = 1;
    
    /// Chain ID for Sepolia Testnet
    pub const SEPOLIA_CHAIN_ID: u64 = 11155111;
    
    /// Chain ID for Goerli Testnet (deprecated)
    pub const GOERLI_CHAIN_ID: u64 = 5;
    
    /// Chain ID for Polygon Mainnet
    pub const POLYGON_CHAIN_ID: u64 = 137;
    
    /// Chain ID for Mumbai Testnet (Polygon)
    pub const MUMBAI_CHAIN_ID: u64 = 80001;
}

/// Filesystem and path constants
pub mod paths {
    /// Relative path to frontend crate
    pub const FRONTEND_PATH: &str = "crates/frontend";
    
    /// Relative path to server crate
    pub const SERVER_PATH: &str = "crates/server";
    
    /// Relative path to smart contracts crate
    pub const SMART_CONTRACTS_PATH: &str = "crates/smart-contracts";
    
    /// Relative path to CLI crate
    pub const CLI_PATH: &str = "crates/cli";
    
    /// Relative path to shared crate
    pub const SHARED_PATH: &str = "crates/shared";
    
    /// Relative path to utils crate
    pub const UTILS_PATH: &str = "crates/utils";
    
    /// Directory for contract artifacts
    pub const ARTIFACTS_DIR: &str = "artifacts";
    
    /// Directory for build outputs
    pub const TARGET_DIR: &str = "target";
}

/// CLI and display constants
pub mod cli {
    /// CLI version string
    pub const VERSION: &str = "0.1.0";
    
    /// CLI title
    pub const TITLE: &str = "Ethereum Boilerplate CLI";
    
    /// CLI subtitle
    pub const SUBTITLE: &str = "Full-stack Web3 development toolkit";
    
    /// Progress bar template
    pub const PROGRESS_BAR_TEMPLATE: &str = 
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({msg})";
    
    /// Progress bar characters
    pub const PROGRESS_CHARS: &str = "█▉▊▋▌▍▎▏  ";
    
    /// Banner line length
    pub const BANNER_LINE_LENGTH: usize = 60;
    /// Banner title width
    pub const BANNER_TITLE_WIDTH: usize = 72;
    /// Banner subtitle width
    pub const BANNER_SUBTITLE_WIDTH: usize = 69;
}

/// Date and time formatting constants
pub mod datetime {
    /// Default date/time format string
    pub const DEFAULT_FORMAT: &str = "%Y-%m-%d %H:%M:%S UTC";
    
    /// Invalid timestamp message
    pub const INVALID_TIMESTAMP: &str = "Invalid timestamp";
}

/// Error messages
pub mod errors {
    /// Generic invalid address message prefix
    pub const INVALID_ADDRESS_PREFIX: &str = "Invalid address";
    
    /// Generic invalid private key message prefix
    pub const INVALID_PRIVATE_KEY_PREFIX: &str = "Invalid private key";
    
    /// Generic invalid amount message prefix
    pub const INVALID_AMOUNT_PREFIX: &str = "Invalid amount";
    
    /// Network timeout error message
    pub const NETWORK_TIMEOUT: &str = "Network health check timeout";
    
    /// Max retries exceeded error message
    pub const MAX_RETRIES_EXCEEDED: &str = "Max retries exceeded";
    
    /// Invalid RPC URL message prefix
    pub const INVALID_RPC_URL_PREFIX: &str = "Invalid RPC URL";
}

/// Validation constants
pub mod validation {
    /// Minimum Ethereum address length (with 0x prefix)
    pub const MIN_ADDRESS_LENGTH: usize = 42;
    
    /// Maximum Ethereum address length (with 0x prefix)
    pub const MAX_ADDRESS_LENGTH: usize = 42;
    
    /// Minimum private key length (with 0x prefix)
    pub const MIN_PRIVATE_KEY_LENGTH: usize = 66;
    
    /// Maximum private key length (with 0x prefix)
    pub const MAX_PRIVATE_KEY_LENGTH: usize = 66;
    
    /// Ethereum address prefix
    pub const ADDRESS_PREFIX: &str = "0x";
    
    /// Private key prefix
    pub const PRIVATE_KEY_PREFIX: &str = "0x";
}
