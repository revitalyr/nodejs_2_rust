//! Logging utilities

use crate::error::Result;

/// Initialize logging with default level
pub fn init_logging(level: &str) -> Result<()> {
    let level = match level {
        "debug" => tracing::Level::DEBUG,
        "trace" => tracing::Level::TRACE,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };
    
    // Simple subscriber without tracing_subscriber for now
    std::println!("Logging initialized at level: {:?}", level);
    Ok(())
}

/// Initialize logging for development
pub fn init_dev_logging() -> Result<()> {
    // Simple dev logging without tracing_subscriber
    std::println!("Dev logging initialized");
    Ok(())
}

/// Initialize logging for production
pub fn init_prod_logging() -> Result<()> {
    // Simple prod logging without tracing_subscriber
    std::println!("Prod logging initialized");
    Ok(())
}

/// Initialize logging for testing
pub fn init_test_logging() -> Result<()> {
    // Simple test logging without tracing_subscriber
    std::println!("Test logging initialized");
    Ok(())
}

/// Get current log level
pub fn get_log_level() -> String {
    std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string())
}

/// Set log level
pub fn set_log_level(level: &str) {
    std::env::set_var("LOG_LEVEL", level);
}

/// Check if debug logging is enabled
pub fn is_debug_enabled() -> bool {
    get_log_level() == "debug" || get_log_level() == "trace"
}

/// Check if trace logging is enabled
pub fn is_trace_enabled() -> bool {
    get_log_level() == "trace"
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        if $crate::logging::is_debug_enabled() {
            tracing::debug!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        if $crate::logging::is_trace_enabled() {
            tracing::trace!($($arg)*);
        }
    };
}
