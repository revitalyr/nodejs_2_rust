//! Shared types, constants, and utilities for Ethereum Boilerplate Rust project.
//! This crate provides a unified interface for server and client applications.

// 1. Declare internal modules
pub mod common_types;
pub mod constants;
pub mod enums;
pub mod types;
pub mod utils;

// 2. Refactor exports: Explicit is better than implicit.
// Instead of *, export only what external users actually need.

pub use constants::*;
pub use enums::*;

// Resolve WalletInfo conflict - export from types, not common_types
pub use types::{
    WalletInfo,
    ERC20Balance,
    NFTBalance,
};

// If common_types has unique items:
pub use common_types::*;

// Utilities are better left in their namespace,
// or exported selectively to avoid confusion with types.
pub use utils::*;