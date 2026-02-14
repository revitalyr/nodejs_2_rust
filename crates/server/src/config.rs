use anyhow::{Context, Result};
use std::env;
use url::Url;

#[derive(Clone)]
pub struct Config {
    /// URL для подключения к Ethereum ноде (строго типизирован)
    pub ethereum_rpc_url: Url,

    /// API ключ Moralis для расширенной аналитики
    pub moralis_api_key: Option<String>,

    /// URL базы данных (PostgreSQL)
    pub database_url: Option<String>,
}

// Кастомная реализация Debug для безопасности (не логируем секреты полностью)
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("ethereum_rpc_url", &self.ethereum_rpc_url.to_string())
            .field("moralis_api_key", &self.moralis_api_key.as_ref().map(|_| "***"))
            .field("database_url", &self.database_url.as_ref().map(|_| "***"))
            .finish()
    }
}

impl Config {
    /// Загружает конфигурацию из переменных окружения
    pub fn from_env() -> Result<Self> {
        // Загружаем .env если он есть
        let _ = dotenvy::dotenv();

        // 1. Загрузка и парсинг RPC URL
        let rpc_url_raw = env::var("ETHEREUM_RPC_URL")
            .context("ETHEREUM_RPC_URL must be set")?;

        let rpc_url = Url::parse(&rpc_url_raw)
            .context("Invalid ETHEREUM_RPC_URL format")?;

        // Проверка протоколов
        match rpc_url.scheme() {
            "http" | "https" | "ws" | "wss" => {},
            _ => anyhow::bail!("RPC URL scheme must be http(s) or ws(s)"),
        }

        // 2. Опциональные параметры
        let moralis_api_key = env::var("MORALIS_API_KEY").ok();
        let database_url = env::var("DATABASE_URL").ok();

        Ok(Self {
            ethereum_rpc_url: rpc_url,
            moralis_api_key,
            database_url,
        })
    }

    /// Геттер для DB URL (используем атрибут, чтобы подавить warning в bin-проектах)
    #[allow(dead_code)]
    pub fn db_url(&self) -> Option<&str> {
        self.database_url.as_deref()
    }
}