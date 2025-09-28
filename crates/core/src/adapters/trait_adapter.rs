use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    pub prompt: String,
    pub max_tokens: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub text: String,
}

#[async_trait]
pub trait LLMAdapter: Send + Sync {
    async fn call(&self, req: LLMRequest) -> Result<LLMResponse, String>;
    fn name(&self) -> &'static str;
}
