#[cfg(test)]
mod tests {
    use axum::{
        http::StatusCode,
        response::IntoResponse,
        routing::{get, post},
        Json, Router,
    };
    use serde_json::{json, Value};
    use tokio::net::TcpListener;

    // --- Помощник для запуска тестового сервера ---
    async fn spawn_app() -> String {
        let app = create_test_app().await;
        // Привязываемся к порту 0, чтобы ОС выделила любой свободный порт
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Не удалось привязать слушателя");
        let addr = listener.local_addr().expect("Не удалось получить локальный адрес");

        tokio::spawn(async move {
            axum::serve(listener, app).await.expect("Ошибка сервера");
        });

        format!("http://{}", addr)
    }



    #[tokio::test]
    async fn test_complete_wallet_workflow() {
        let base_url = spawn_app().await;
        let client = reqwest::Client::new();
        let wallet_address = "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b";

        // 1. Проверка баланса (Initial Balance)
        let response = client
            .get(format!("{}/api/balances/wallet", base_url))
            .query(&[("address", wallet_address)])
            .send()
            .await
            .expect("Ошибка отправки запроса");

        assert_eq!(response.status().as_u16(), StatusCode::OK.as_u16());

        // 2. Деплой контракта (Deploy)
        let deploy_res = client
            .post(format!("{}/api/deploy-contract", base_url))
            .json(&json!({
                "contract_type": "ERC20",
                "name": "TestToken",
                "symbol": "TST"
            }))
            .send()
            .await
            .expect("Ошибка деплоя");

        let body: Value = deploy_res.json().await.unwrap();
        let contract_address = body["address"].as_str().expect("Адрес не найден");
        assert!(contract_address.starts_with("0x"));

        // 3. Минтинг токенов (Mint)
        let mint_res = client
            .post(format!("{}/api/mint-tokens", base_url))
            .json(&json!({
                "contract_address": contract_address,
                "amount": "1000"
            }))
            .send()
            .await
            .expect("Ошибка минтинга");

        assert_eq!(mint_res.status().as_u16(), 200);

        // 4. История транзакций (History)
        let history_res = client
            .get(format!("{}/api/transactions", base_url))
            .query(&[("address", wallet_address), ("limit", "5")])
            .send()
            .await
            .expect("Ошибка получения истории");

        assert_eq!(history_res.status().as_u16(), StatusCode::OK.as_u16());
    }

    #[tokio::test]
    async fn test_concurrent_requests() {
        let base_url = spawn_app().await;
        let client = reqwest::Client::new();
        let wallet_address = "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b";

        // Используем join_all для параллельного выполнения запросов
        let tasks: Vec<_> = (0..10).map(|_| {
            let client = client.clone();
            let url = format!("{}/api/balances/wallet", base_url);
            let address = wallet_address.to_string();

            async move {
                client.get(url)
                    .query(&[("address", address)])
                    .send()
                    .await
                    .unwrap()
                    .status()
                    .as_u16()
            }
        }).collect();

        let results = futures::future::join_all(tasks).await;

        for status in results {
            assert_eq!(status, 200);
        }
    }

    // --- Реализация тестового приложения ---
    async fn create_test_app() -> Router {
        // Заглушки обработчиков
        async fn mock_deploy() -> impl IntoResponse {
            (StatusCode::OK, Json(json!({
                "address": "0x1234567890123456789012345678901234567890",
                "transaction_hash": "0xabc"
            })))
        }

        async fn mock_action() -> impl IntoResponse {
            (StatusCode::OK, Json(json!({"success": true})))
        }

        async fn mock_balance() -> impl IntoResponse {
            (StatusCode::OK, Json(json!({
                "balance": "100.0",
                "transactions": [
                    {"hash": "0x1", "value": "1.0"},
                    {"hash": "0x2", "value": "0.5"}
                ]
            })))
        }

        Router::new()
            .route("/api/deploy-contract", post(mock_deploy))
            .route("/api/mint-tokens", post(mock_action))
            .route("/api/transfer-tokens", post(mock_action))
            .route("/api/balances/wallet", get(mock_balance))
            .route("/api/balances/erc20", get(mock_balance))
            .route("/api/transactions", get(mock_balance))
    }
}