use super::trait_adapter::{parse_artifacts_from_text, LLMAdapter, LLMRequest};
use crate::backend::{Artifacts, Backend, BackendError};
use async_trait::async_trait;
use reqwest::blocking::Client;
use reqwest::Client as AsyncClient;
use serde_json::{json, Value};

pub struct OllamaAdapter {
    pub url: String,
    pub api_key: Option<String>,
}

impl OllamaAdapter {
    pub fn new(url: impl Into<String>, api_key: Option<String>) -> Self {
        OllamaAdapter {
            url: url.into(),
            api_key,
        }
    }
}

impl Backend for OllamaAdapter {
    fn generate(&self, prompt: &str, options: Value) -> Result<Artifacts, BackendError> {
        let client = Client::builder()
            .build()
            .map_err(|e| BackendError::Network(e.to_string()))?;
        let mut req = client.post(format!("{}/generate", self.url)).json(&json!({
            "prompt": prompt,
            "options": options,
        }));
        if let Some(k) = &self.api_key {
            req = req.header("Authorization", format!("Bearer {}", k));
        }
        let resp = req
            .send()
            .map_err(|e| BackendError::Network(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(BackendError::Protocol(format!("status {}", resp.status())));
        }
        let v: Value = resp
            .json()
            .map_err(|e| BackendError::Protocol(e.to_string()))?;
        Ok(Artifacts {
            artifacts: v["artifacts"].as_array().cloned().unwrap_or_default(),
        })
    }
}

#[async_trait]
impl LLMAdapter for OllamaAdapter {
    async fn call(
        &self,
        req: LLMRequest,
    ) -> Result<Vec<crate::artifact::ArtifactMetadata>, String> {
        // Basic async implementation: POST to {url}/generate with prompt and options.
        // This is intentionally minimal: callers should extend parsing per their API.
        let client = AsyncClient::builder()
            .build()
            .map_err(|e| format!("client build error: {}", e))?;

        let mut builder = client.post(format!("{}/generate", self.url)).json(&json!({
            "prompt": req.prompt,
            "max_tokens": req.max_tokens,
        }));
        if let Some(k) = &self.api_key {
            builder = builder.header("Authorization", format!("Bearer {}", k));
        }

        let resp = builder
            .send()
            .await
            .map_err(|e| format!("network error: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("status {}", resp.status()));
        }

        let v: Value = resp
            .json()
            .await
            .map_err(|e| format!("invalid json: {}", e))?;

        // If the response has a `text` field, prefer it; otherwise stringify. Then parse into artifacts.
        let text = if let Some(t) = v.get("text").and_then(|v| v.as_str()) {
            t.to_string()
        } else {
            v.to_string()
        };
        Ok(parse_artifacts_from_text(&text))
    }

    fn name(&self) -> &'static str {
        "ollama"
    }
}
