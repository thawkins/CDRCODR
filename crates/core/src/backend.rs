use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Artifacts {
    pub artifacts: Vec<serde_json::Value>,
}

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("network error: {0}")]
    Network(String),
    #[error("protocol error: {0}")]
    Protocol(String),
}

pub trait Backend: Send + Sync {
    fn generate(&self, prompt: &str, options: serde_json::Value) -> Result<Artifacts, BackendError>;
}
