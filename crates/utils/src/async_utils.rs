//! Async utilities and patterns for Web3 infrastructure

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;
use crate::error::{Result, UtilsError};

// FIXED: Now returns String as expected by UtilsError::Internal
fn request_error(msg: String) -> UtilsError {
    UtilsError::Internal(msg)
}

fn timeout_error() -> UtilsError {
    UtilsError::Internal("Operation timed out".to_string())
}

/// Execute list of futures in parallel and collect results.
pub async fn try_join_all<T, E>(
    futures: Vec<Pin<Box<dyn Future<Output = std::result::Result<T, E>> + Send>>>
) -> Result<Vec<T>>
where
    E: std::fmt::Display + 'static,
{
    futures::future::try_join_all(futures)
        .await
        // FIXED: Removed extra anyhow!, use direct error conversion to String
        .map_err(|e| UtilsError::Internal(e.to_string()))
}

/// Execute future with time limit.
pub async fn with_timeout<F, T>(
    future: F,
    timeout: Duration,
) -> Result<T>
where
    F: Future<Output = T>,
{
    tokio::time::timeout(timeout, future)
        .await
        .map_err(|_| timeout_error())
}

// --- Rate Limiting & Flow Control ---

pub struct Throttler {
    delay: Duration,
    last_call: Option<tokio::time::Instant>,
}

impl Throttler {
    pub fn new(delay: Duration) -> Self {
        Self { delay, last_call: None }
    }

    pub async fn throttle<F, T>(&mut self, operation: F) -> T
    where
        F: Future<Output = T>,
    {
        if let Some(last) = self.last_call {
            let elapsed = last.elapsed();
            if elapsed < self.delay {
                sleep(self.delay - elapsed).await;
            }
        }

        let result = operation.await;
        self.last_call = Some(tokio::time::Instant::now());
        result
    }
}

// --- Circuit Breaker ---

pub struct CircuitBreaker {
    failure_threshold: usize,
    failure_count: usize,
    reset_timeout: Duration,
    last_failure: Option<tokio::time::Instant>,
    state: CircuitState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, reset_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            failure_count: 0,
            reset_timeout,
            last_failure: None,
            state: CircuitState::Closed,
        }
    }

    pub async fn execute<F, T, E>(&mut self, operation: F) -> Result<T>
    where
        F: Future<Output = std::result::Result<T, E>>,
        E: std::fmt::Display,
    {
        self.update_state();

        if self.state == CircuitState::Open {
            return Err(request_error("Circuit breaker is currently OPEN".into()));
        }

        match operation.await {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(e) => {
                let err_msg = e.to_string();
                self.on_failure();
                Err(request_error(err_msg))
            }
        }
    }

    fn update_state(&mut self) {
        if self.state == CircuitState::Open {
            if let Some(last) = self.last_failure {
                if last.elapsed() > self.reset_timeout {
                    self.state = CircuitState::HalfOpen;
                }
            }
        }
    }

    fn on_success(&mut self) {
        self.failure_count = 0;
        self.state = CircuitState::Closed;
    }

    fn on_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(tokio::time::Instant::now());

        if self.failure_count >= self.failure_threshold {
            self.state = CircuitState::Open;
        }
    }
}

// --- Batch Processing ---

pub struct BatchProcessor {
    batch_size: usize,
    interval: Duration,
}

impl BatchProcessor {
    pub fn new(batch_size: usize, interval: Duration) -> Self {
        Self { batch_size, interval }
    }

    pub async fn process_batch<T, F, R, E>(
        &self,
        items: Vec<T>,
        processor: F,
    ) -> Result<Vec<R>>
    where
        T: Clone,
        F: Fn(Vec<T>) -> Pin<Box<dyn Future<Output = std::result::Result<Vec<R>, E>> + Send>>,
        E: std::fmt::Display + 'static,
    {
        let mut results = Vec::new();

        for chunk in items.chunks(self.batch_size) {
            let chunk_data = chunk.to_vec();
            let batch_res = processor(chunk_data).await
                .map_err(|e| request_error(e.to_string()))?;

            results.extend(batch_res);
            sleep(self.interval).await;
        }

        Ok(results)
    }
}