#[cfg(test)]
mod tests {
    use alloy::primitives::{Address, U256};
use alloy::primitives::aliases::B256;
use alloy::primitives::utils::format_ether;
    use serde_json::json;
    use std::str::FromStr;
    use reqwest::Url;

    use ethereum_boilerplate_server::{
        config::Config,
        error::AppError,
    };

    // ИСПРАВЛЕНО: Добавлены два символа 'b5', чтобы длина стала ровно 40 символов
    const TEST_ADDR: &str = "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b5";

    #[test]
    fn test_address_parsing_robustness() {
        let parsed = Address::from_str(TEST_ADDR).expect("Address must be valid");
        
        // Корректное преобразование: только хеш-часть в верхний регистр
        let uppercase_addr = format!("0x{}", &TEST_ADDR[2..].to_uppercase());
        let parsed_upper = Address::from_str(&uppercase_addr).expect("Should parse uppercase hex part");
        
        assert_eq!(parsed, parsed_upper);
        assert_eq!(parsed.as_slice().len(), 20);
    }

    #[test]
    fn test_eth_conversions() {
        let val = U256::from(10).pow(U256::from(17)); // 0.1 ETH
        
        // Точное сравнение с 18 знаками
        assert_eq!(format_ether(val), "0.100000000000000000");
        
        // Дополнительная проверка через парсинг
        let back_to_wei = U256::from_str("100000000000000000").unwrap();
        assert_eq!(val, back_to_wei);

        // Gwei → ETH (1 Gwei = 10^9 Wei)
        let gwei = U256::from(1_000_000_000);
        assert_eq!(format_ether(gwei), "0.000000001000000000");
    }

    #[test]
    fn test_h256_hash_validation() {
        let raw_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890123456";
        let hash = B256::from_str(raw_hash).expect("Hash parsing failed");

        assert_eq!(hash.as_slice().len(), 32);
        assert_eq!(format!("{:x}", hash), raw_hash.trim_start_matches("0x").to_lowercase());
    }

    #[test]
    fn test_config_initialization() {
        let rpc_url_str = "https://eth-mainnet.g.alchemy.com/v2/key";
        let config = Config {
            ethereum_rpc_url: Url::parse(rpc_url_str).expect("Failed to parse URL"),
            moralis_api_key: Some("secret_key".into()),
            database_url: None,
        };

        assert!(config.ethereum_rpc_url.as_str().contains("alchemy"));
    }

    #[test]
    fn test_app_error_variant_matching() {
        let msg = "Database connection timed out";
        let err = AppError::Internal(anyhow::anyhow!(msg));

        assert!(matches!(err, AppError::Internal(_)));
        assert!(err.to_string().contains(msg));
    }

    #[test]
    fn test_abi_json_structure() {
        let abi_json = json!([
            {
                "constant": true,
                "inputs": [{"name": "_owner", "type": "address"}],
                "name": "balanceOf",
                "outputs": [{"name": "balance", "type": "uint256"}],
                "type": "function"
            }
        ]);

        assert_eq!(abi_json[0]["name"], "balanceOf");
    }
}
