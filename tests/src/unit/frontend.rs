#[cfg(test)]
mod tests {
    use leptos::prelude::*; // Включает signal(), ReadSignal, WriteSignal и т.д.
    use serde::{Deserialize, Serialize};
    use wasm_bindgen_test::*;

    // Конфигурация для запуска в браузере через wasm-pack test
    wasm_bindgen_test_configure!(run_in_browser);

    // --- Domain Types -----------------------------------------------------------

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct Transaction {
        pub hash: String,
        pub from: String,
        pub to: Option<String>,
        pub value: String,
        pub gas_used: String,
        pub gas_price: Option<String>,
        pub block_number: u64,
        pub status: TransactionStatus,
    }

    #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
    pub enum TransactionStatus {
        Success,
        Pending,
        Failed,
    }

    // --- Helpers (Логика форматирования) -----------------------------------------

    fn shorten_address(addr: &str) -> String {
        if addr.len() > 10 {
            format!("{}...{}", &addr[..6], &addr[addr.len() - 4..])
        } else {
            addr.to_string()
        }
    }

    fn calculate_gas_fee(gas_used: &str, gas_price: Option<&str>) -> f64 {
        let gas_used_num: f64 = gas_used.parse().unwrap_or(0.0);
        let gas_price_num: f64 = gas_price.and_then(|p| p.parse().ok()).unwrap_or(0.0);
        // Расчет: Gas * Gwei / 1e9
        (gas_used_num * gas_price_num) / 1_000_000_000.0
    }

    // --- Tests ------------------------------------------------------------------



    #[wasm_bindgen_test]
    async fn test_wallet_connection_reactive() {
        // ИСПРАВЛЕНО: Используем signal() вместо устаревшего create_signal()
        let (wallet_connected, set_wallet_connected) = signal(false);
        let (current_address, set_current_address) = signal(String::new());

        assert!(!wallet_connected.get(), "Начальное состояние должно быть false");

        // Установка значений
        set_wallet_connected.set(true);
        set_current_address.set("0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b".to_string());

        assert!(wallet_connected.get());

        // .with() позволяет заглянуть внутрь значения без его клонирования
        current_address.with(|addr| {
            assert_eq!(addr, "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b");
        });
    }

    #[wasm_bindgen_test]
    async fn test_gas_calculation_logic() {
        let fee = calculate_gas_fee("21000", Some("20"));
        assert_eq!(fee, 0.00042);

        let fee_zero = calculate_gas_fee("21000", None);
        assert_eq!(fee_zero, 0.0);
    }

    #[wasm_bindgen_test]
    async fn test_transaction_history_signal() {
        // В Leptos 0.7 сигналы автоматически очищаются. Явно указываем тип коллекции.
        let (transactions, set_transactions) = signal(Vec::<Transaction>::new());

        let mock_tx = Transaction {
            hash: "0xhash".to_string(),
            from: "0xfrom".to_string(),
            to: None,
            value: "1.0".to_string(),
            gas_used: "21000".to_string(),
            gas_price: Some("10".to_string()),
            block_number: 100,
            status: TransactionStatus::Success,
        };

        // .update() изменяет значение по месту, не вызывая клонирование всего вектора
        set_transactions.update(|txs| txs.push(mock_tx));

        // Проверяем реактивность через .get()
        assert_eq!(transactions.get().len(), 1);
        assert_eq!(transactions.get()[0].status, TransactionStatus::Success);
    }

    #[wasm_bindgen_test]
    async fn test_address_formatter() {
        let addr = "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b";
        assert_eq!(shorten_address(addr), "0x742d...4d8b");
    }

    #[wasm_bindgen_test]
    async fn test_signal_options_transition() {
        let (data, set_data) = signal(None::<i32>);

        assert!(data.get().is_none());

        set_data.set(Some(100));
        assert_eq!(data.get(), Some(100));

        set_data.set(None);
        assert!(data.get().is_none());
    }
}