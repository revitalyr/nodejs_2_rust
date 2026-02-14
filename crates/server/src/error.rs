use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Ethereum provider error: {0}")]
    EthereumProvider(String),

    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
}

/// Унифицированный JSON-ответ для фронтенда
#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl AppError {
    /// Хелпер для 404 Not Found
    #[allow(dead_code)] // Подавляем warning, пока не используется в хендлерах
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    /// Хелпер для 400 Bad Request
    #[allow(dead_code)]
    pub fn invalid_address(addr: impl Into<String>) -> Self {
        Self::InvalidAddress(addr.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Логируем ошибку для внутреннего мониторинга (stdout/Sentry/etc)
        tracing::error!(target: "server_error", "Error details: {:?}", self);

        let (status, message) = match &self {
            // Клиентские ошибки: детали можно показывать пользователю
            AppError::InvalidAddress(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::ParseError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),

            // Ошибки инфраструктуры: скрываем подробности реализации
            AppError::EthereumProvider(_) => (StatusCode::BAD_GATEWAY, "Blockchain node connection failure".into()),
            AppError::HttpClient(_) => (StatusCode::BAD_GATEWAY, "External API service unavailable".into()),
            AppError::Database(_) | AppError::Migration(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database operation failed".into())
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred".into()),
        };

        // Включаем детали ошибки только если мы НЕ в релизной сборке
        let details = if cfg!(debug_assertions) {
            Some(self.to_string())
        } else {
            None
        };

        let body = Json(ErrorResponse {
            error: message,
            status: status.as_u16(),
            details,
        });

        (status, body).into_response()
    }
}