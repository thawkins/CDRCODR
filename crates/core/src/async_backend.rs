use serde_json::Value;
use crate::backend::Artifacts;

#[async_trait::async_trait]
pub trait AsyncBackend: Send + Sync {
    async fn generate(&self, prompt: &str, options: Value) -> Result<Artifacts, crate::backend::BackendError>;
}

// A small async mock adapter for the spike. Not wired into the rest of the code yet.
pub struct AsyncMockAdapter {
    pub response: Value,
}

impl AsyncMockAdapter {
    pub fn new(response: Value) -> Self {
        Self { response }
    }
}

#[async_trait::async_trait]
impl AsyncBackend for AsyncMockAdapter {
    async fn generate(&self, _prompt: &str, _options: Value) -> Result<Artifacts, crate::backend::BackendError> {
        // Simulate async delay if present
        if let Some(ms) = self.response.get("timeout_ms").and_then(|v| v.as_u64()) {
            let ms = std::cmp::min(ms, 10_000);
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
        }
        if let Some(arr) = self.response.get("artifacts").and_then(|v| v.as_array()) {
            return Ok(Artifacts { artifacts: arr.clone() });
        }
        // fallback
        let art = serde_json::json!({ "path": "async/mock.txt", "summary": "async mock", "content": "ok" });
        Ok(Artifacts { artifacts: vec![art] })
    }
}
