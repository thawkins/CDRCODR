use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::artifact::ArtifactMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    pub prompt: String,
    pub max_tokens: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub text: String,
}

// Parse adapter textual output into zero-or-more artifacts.
// - If the text parses as a JSON array of objects with { path, content } -> map them
// - If the text parses as a JSON object with content (and optional path) -> map single
// - Otherwise return a single artifact containing the raw text
pub fn parse_artifacts_from_text(text: &str) -> Vec<ArtifactMetadata> {
    // helper to produce a short summary from content
    fn summarize(s: &str) -> String {
        s.lines().next().unwrap_or("").chars().take(120).collect()
    }

    if let Ok(val) = serde_json::from_str::<Value>(text) {
        if let Some(arr) = val.as_array() {
            let mut out = Vec::new();
            for (i, it) in arr.iter().enumerate() {
                if let Some(obj) = it.as_object() {
                    let path = obj.get("path").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| format!("generated/artifact-{}.txt", i + 1));
                    let content_opt = obj.get("content").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let summary = content_opt.as_deref().map(summarize).unwrap_or_else(|| obj.get("summary").and_then(|v| v.as_str()).unwrap_or("").to_string());
                    if let Some(c) = content_opt.clone() {
                        out.push(ArtifactMetadata { path, summary: summarize(&c), checksum: None, content: Some(c) });
                        continue;
                    }
                    if !summary.is_empty() {
                        out.push(ArtifactMetadata { path, summary, checksum: None, content: None });
                        continue;
                    }
                }
                // fallback for non-object array items
                out.push(ArtifactMetadata { path: format!("generated/artifact-{}.json", i + 1), summary: it.to_string().chars().take(120).collect(), checksum: None, content: Some(it.to_string()) });
            }
            if !out.is_empty() {
                return out;
            }
        } else if val.is_object() {
            if let Some(obj) = val.as_object() {
                if let Some(content) = obj.get("content").and_then(|v| v.as_str()) {
                    let path = obj.get("path").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| "generated/artifact-1.txt".to_string());
                    return vec![ArtifactMetadata { path, summary: summarize(content), checksum: None, content: Some(content.to_string()) }];
                }
                if let Some(summary) = obj.get("summary").and_then(|v| v.as_str()) {
                    let path = obj.get("path").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| "generated/artifact-1.txt".to_string());
                    return vec![ArtifactMetadata { path, summary: summary.to_string(), checksum: None, content: None }];
                }
            }
        }
    }
    vec![ArtifactMetadata { path: "generated/artifact-1.txt".to_string(), summary: summarize(text), checksum: None, content: Some(text.to_string()) }]
}

#[async_trait]
pub trait LLMAdapter: Send + Sync {
    // Return zero-or-more artifacts parsed from the adapter response
    async fn call(&self, req: LLMRequest) -> Result<Vec<ArtifactMetadata>, String>;
    fn name(&self) -> &'static str;
}
