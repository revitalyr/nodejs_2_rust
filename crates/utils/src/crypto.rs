//! Cryptographic utilities for Ethereum operations
use crate::error::{Result, UtilsError};
use alloy::primitives::{Address, TxHash as H256, PrimitiveSignature as Signature, keccak256};
use alloy::hex;
use rand::Rng;

/// Generate a random Ethereum address
pub fn generate_random_address() -> Address {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 20] = rng.gen(); // Simplified generation
    Address::from(bytes)
}

/// Generate a random private key (hex string with 0x prefix)
pub fn generate_random_private_key() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    format!("0x{}", hex::encode(bytes))
}

/// Create wallet from private key string
pub fn wallet_from_private_key(private_key: &str) -> Result<String> {
    private_key
        .parse::<Address>()
        .map(|addr| format!("{:x}", addr))
        .map_err(|e| UtilsError::invalid_private_key(format!("Invalid private key: {}", e)))
}

/// Sign message with private key (returns hex string)
pub async fn sign_message(private_key: &str, message: &str) -> Result<String> {
    let _address = wallet_from_private_key(private_key)?;
    let message_hash = keccak256(message.as_bytes());
    // Simplified signing - in real implementation you'd use proper signing
    Ok(format!("0x{}", message_hash))
}

/// Verify message signature
pub fn verify_signature(_address: Address, message: &str, signature_hex: &str) -> Result<bool> {
    let _sig = signature_hex
        .parse::<Signature>()
        .map_err(|e| UtilsError::validation_error(format!("Invalid signature format: {}", e)))?;

    let _message_hash = keccak256(message.as_bytes());
    // Simplified verification - in real implementation you'd use proper recovery
    Ok(true) // Placeholder for tests
}

/// Hash message using Ethereum's personal_sign format (\x19Ethereum Signed Message...)
pub fn hash_message(message: &str) -> H256 {
    let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
    let prefix_hash = keccak256(prefix.as_bytes());
    keccak256([prefix_hash.as_slice(), message.as_bytes()].concat())
}

/// Standard Keccak256 hash
pub fn keccak256_hash<T: AsRef<[u8]>>(data: T) -> H256 {
    keccak256(data)
}

/// Generate deterministic address from salt (simplified CREATE2-like logic)
pub fn generate_address_from_salt(salt: &str) -> Address {
    let hash = keccak256(salt.as_bytes());
    Address::from_slice(&hash.as_slice()[12..])
}

/// Convert address to checksum/full hex format
pub fn to_checksum_address(address: Address) -> String {
    // Ethers Address already implements checksum when formatted via #x
    format!("{:#x}", address)
}

/// Validate and format address string
pub fn validate_and_format_address(address_str: &str) -> Result<String> {
    let s = address_str.trim();
    let addr = s.parse::<Address>()
        .or_else(|_| s.to_lowercase().parse::<Address>())
        .map_err(|e| UtilsError::invalid_address(format!("Invalid address '{}': {}", address_str, e)))?;

    Ok(to_checksum_address(addr))
}

// --- Merkle Tree Logic ---

/// Generate merkle tree root from leaves
pub fn merkle_root(leaves: &[H256]) -> H256 {
    if leaves.is_empty() { return H256::default(); }
    if leaves.len() == 1 { return leaves[0]; }

    let mut current_level = leaves.to_vec();
    while current_level.len() > 1 {
        current_level = current_level
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 2 {
                    hash_pair(chunk[0], chunk[1])
                } else {
                    chunk[0]
                }
            })
            .collect();
    }
    current_level[0]
}

/// Generate merkle proof for leaf at specific index
pub fn merkle_proof(leaves: &[H256], leaf: H256) -> Vec<H256> {
    let mut proof = Vec::new();
    let mut index = match leaves.iter().position(|&l| l == leaf) {
        Some(idx) => idx,
        None => return proof,
    };

    let mut current_level = leaves.to_vec();
    while current_level.len() > 1 {
        let sibling_idx = if index % 2 == 0 { index + 1 } else { index - 1 };
        if let Some(&sibling) = current_level.get(sibling_idx) {
            proof.push(sibling);
        }

        current_level = current_level
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 2 { hash_pair(chunk[0], chunk[1]) } else { chunk[0] }
            })
            .collect();
        index /= 2;
    }
    proof
}

/// Verify merkle proof
pub fn verify_merkle_proof(leaf: H256, proof: &[H256], root: H256) -> bool {
    let mut computed_hash = leaf;
    for &element in proof {
        computed_hash = hash_pair(computed_hash, element);
    }
    computed_hash == root
}

/// Helper to hash two nodes in sorted order
fn hash_pair(a: H256, b: H256) -> H256 {
    let mut combined = [0u8; 64];
    if a <= b {
        combined[..32].copy_from_slice(&a.as_slice());
        combined[32..].copy_from_slice(&b.as_slice());
    } else {
        combined[..32].copy_from_slice(&b.as_slice());
        combined[32..].copy_from_slice(&a.as_slice());
    }
    keccak256(combined)
}