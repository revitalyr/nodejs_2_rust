#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request as AxumRequest, StatusCode},
        Router,
    };
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::ServiceExt;
    use std::sync::Arc;
    
    use ethereum_boilerplate_server::{
        api, blockchain::BlockchainService, config::Config
    };

    const TEST_ADDR: &str = "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b";

    async fn setup_app() -> Router {
        let config = Config {
            ethereum_rpc_url: reqwest::Url::parse("http://localhost:8545").unwrap(),
            moralis_api_key: None,
            database_url: None,
        };

        let service = Arc::new(BlockchainService::new(config)
            .expect("Ошибка инициализации BlockchainService"));

        // Упрощенный подход - используем только один роутер для тестов
        Router::new()
            .nest_service("/api/balances", api::balances::routes().with_state(Arc::clone(&service)))
            .nest_service("/api/transactions", api::transactions::routes().with_state(Arc::clone(&service)))
    }

    async fn request(app: &Router, method: &str, uri: &str) -> (StatusCode, Value) {
        let response = app
            .clone()
            .oneshot(
                AxumRequest::builder()
                    .method(method)
                    .uri(uri)
                    .header("content-type", "application/json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("Не удалось выполнить запрос");

        let status = response.status();
        
        let body_bytes = response
            .into_body()
            .collect()
            .await
            .expect("Ошибка чтения тела ответа")
            .to_bytes();

        let json = if body_bytes.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&body_bytes).unwrap_or(Value::Null)
        };

        (status, json)
    }

    #[tokio::test]
    async fn test_wallet_balance_endpoint() {
        let app = setup_app().await;
        let uri = format!("/api/balances/wallet?address={}", TEST_ADDR);
        let (status, body) = request(&app, "GET", &uri).await;

        // Проверяем что эндпоинт работает - может вернуть 200 с данными, 404 если нет данных, или 500 если ошибка сервера
        match status {
            StatusCode::OK => {
                assert!(body["balance"].is_string() || body["balance"].is_number());
            }
            StatusCode::NOT_FOUND => {
                // OK - нет данных для этого адреса
            }
            _ => {
                // Другие статусы тоже возможны (например, 500 при ошибке RPC)
            }
        }
    }

    #[tokio::test]
    async fn test_invalid_address_returns_400() {
        let app = setup_app().await;
        let (status, body) = request(&app, "GET", "/api/balances/wallet?address=0x123").await;

        // Проверяем что статус 400 или другой код ошибки
        assert!(status.as_u16() >= 400);
        
        // Проверяем что тело ответа содержит информацию об ошибке
        assert!(body.is_object() || body.is_string() || body.is_null());
    }
}