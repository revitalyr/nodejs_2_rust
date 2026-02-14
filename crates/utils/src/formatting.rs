//! Formatting utilities for Ethereum values

use alloy::primitives::U256;
use ethereum_boilerplate_shared::{datetime::DEFAULT_FORMAT, datetime::INVALID_TIMESTAMP};

/// Formats Wei amount to ETH string
pub fn format_wei(wei: U256) -> String {
    let wei_f64 = wei.to_string().parse::<f64>().unwrap_or(0.0);
    format!("{:.6}", wei_f64 / 1e18)
}

/// Formats ETH amount to readable string
pub fn format_eth(eth: f64) -> String {
    format!("{:.6}", eth)
}

/// Parses ETH string to Wei
pub fn parse_eth(eth: &str) -> Result<U256, Box<dyn std::error::Error>> {
    let eth_value: f64 = eth.parse()?;
    let wei_value = (eth_value * 1e18) as u128;
    Ok(U256::from(wei_value))
}

/// Parses Wei string to U256
pub fn parse_wei(wei: &str) -> Result<U256, Box<dyn std::error::Error>> {
    wei.parse::<U256>().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

/// Formats address with proper checksum
pub fn format_address(address: &str) -> String {
    if let Ok(addr) = address.parse::<alloy::primitives::Address>() {
        format!("{:#x}", addr)
    } else {
        address.to_string()
    }
}

/// Formats transaction hash
pub fn format_tx_hash(hash: &str) -> String {
    if let Ok(h) = hash.parse::<alloy::primitives::TxHash>() {
        format!("{:#x}", h)
    } else {
        hash.to_string()
    }
}

/// Formats gas price in Gwei
pub fn format_gas_price(gas_price: U256) -> String {
    let gwei = gas_price.to::<u128>() as f64 / 1_000_000_000.0;
    format!("{:.2} Gwei", gwei)
}

/// Formats block number
pub fn format_block_number(block_number: u64) -> String {
    format!("#{}", block_number)
}

/// Formats timestamp to readable date
pub fn format_timestamp(timestamp: u64) -> String {
    chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
        .map_or_else(|| INVALID_TIMESTAMP.to_string(), |dt| dt.format(DEFAULT_FORMAT).to_string())
}

/// Formats duration in human readable format
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else if seconds < 86400 {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    } else {
        format!("{}d {}h", seconds / 86400, (seconds % 86400) / 3600)
    }
}

/// Formats USD value with proper formatting
pub fn format_usd(value: f64) -> String {
    if value < 0.01 {
        format!("${:.6}", value)
    } else if value < 1.0 {
        format!("${:.4}", value)
    } else {
        let sign = if value < 0.0 { "-" } else { "" };
        let v = value.abs();
        let whole = v.trunc() as i64;
        let frac = (v.fract() * 100.0).round() as i64;

        let s = whole.to_string();
        let rev_chars: Vec<char> = s.chars().rev().collect();
        let mut with_commas = String::new();
        for (i, ch) in rev_chars.iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                with_commas.push(',');
            }
            with_commas.push(*ch);
        }
        let whole_commas: String = with_commas.chars().rev().collect();

        format!("${}{}.{:02}", sign, whole_commas, frac)
    }
}

/// Formats percentage
pub fn format_percentage(value: f64) -> String {
    if value >= 0.0 {
        format!("+{:.2}%", value)
    } else {
        format!("{:.2}%", value)
    }
}

/// Truncates string with ellipsis
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length.saturating_sub(3)])
    }
}

/// Formats address for display (truncated)
pub fn format_address_display(address: &str) -> String {
    if address.len() <= 10 {
        address.to_string()
    } else {
        // match expected display: first 7 chars and last 3 chars
        format!("{}...{}", &address[..7], &address[address.len()-3..])
    }
}

/// Formats transaction hash for display
pub fn format_tx_hash_display(hash: &str) -> String {
    if hash.len() <= 16 {
        hash.to_string()
    } else {
        format!("{}...{}", &hash[..10], &hash[hash.len()-6..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_wei() {
        let wei = U256::from(1000000000000000000u64);
        let formatted = format_wei(wei);
        assert_eq!(formatted, "1.000000");
    }

    #[test]
    fn test_format_eth() {
        let formatted = format_eth(1.23456789);
        assert_eq!(formatted, "1.234568");
    }

    #[test]
    fn test_parse_eth() {
        let parsed = parse_eth("1.5").unwrap();
        assert_eq!(parsed, U256::from(1500000000000000000u64));
    }

    #[test]
    fn test_format_address_display() {
        let address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        let formatted = format_address_display(address);
        assert_eq!(formatted, "0xf39fd...266");
    }

    #[test]
    fn test_format_usd() {
        assert_eq!(format_usd(1.2345), "$1.23");
        assert_eq!(format_usd(0.001), "$0.001000");
        assert_eq!(format_usd(1234.567), "$1,234.57");
    }

    #[test]
    fn test_format_percentage() {
        assert_eq!(format_percentage(2.5), "+2.50%");
        assert_eq!(format_percentage(-1.5), "-1.50%");
    }
}
