//! Centralized error handling utilities

use crate::error::{Result, UtilsError};

/// Convert any displayable error into `UtilsError::Parse`
pub fn to_parse_error<E: std::fmt::Display>(error: E) -> UtilsError {
    UtilsError::parse(error.to_string())
}

/// Convert a timeout into `UtilsError::Parse`
pub fn timeout_error() -> UtilsError {
    UtilsError::parse("Timeout")
}

/// Convert a generic request error into `UtilsError::Parse`
pub fn request_error() -> UtilsError {
    UtilsError::parse("Request error")
}

/// Convert a validation error into `UtilsError::Validation`
pub fn validation_error<E: std::fmt::Display>(error: E) -> UtilsError {
    UtilsError::validation_error(error.to_string())
}

/// Convert a configuration error into `UtilsError::Config`
pub fn config_error<E: std::fmt::Display>(error: E) -> UtilsError {
    UtilsError::config_error(error.to_string())
}

/// Convert a network-related error into a `UtilsError` variant.
/// Note: constructing `reqwest::Error` from arbitrary types isn't possible here,
/// so we map network issues to a parse/diagnostic error for now.
pub fn network_error<E: std::fmt::Display>(error: E) -> UtilsError {
    UtilsError::parse(format!("Network error: {}", error))
}

/// Map a `Result<T, E>` into the crate `Result<T>` using `to_parse_error`
pub fn map_to_parse_error<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> Result<T> {
    result.map_err(to_parse_error)
}

/// Map a `tokio::time::error::Elapsed` (timeout) into `UtilsError`
pub fn map_timeout_error<T>(result: std::result::Result<T, tokio::time::error::Elapsed>) -> Result<T> {
    result.map_err(|_| timeout_error())
}

/// Map a generic request error into `UtilsError`
pub fn map_request_error<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> Result<T> {
    result.map_err(|_| request_error())
}

/// Map a validation error into `UtilsError::Validation`
pub fn map_validation_error<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> Result<T> {
    result.map_err(validation_error)
}

/// Map a configuration error into `UtilsError::Config`
pub fn map_config_error<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> Result<T> {
    result.map_err(config_error)
}
