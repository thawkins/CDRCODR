// The file continues with a full-featured Backend-based MockAdapter used by the CLI
// and tests. Below we also provide a small LLM-facing wrapper `LLMMockAdapter`
// that implements the `LLMAdapter` trait without colliding with the Backend
// MockAdapter type.
use crate::backend::{Artifacts, Backend, BackendError};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub struct MockAdapter {
    // optional pre-seeded response. Can be:
    // - an object with `artifacts` or `error` or `timeout_ms` or `partial`
    // - an array of such objects to be returned sequentially across generate() calls
    pub seeded: Option<Value>,
    // index for sequential seeds
    seq_index: Mutex<usize>,
    // total generate() call count for intermittent scenarios
    call_count: Mutex<usize>,
    // optional path to a persisted mock state file (stores call count)
    state_path: Option<PathBuf>,
}

// Lightweight adapter implementing the LLMAdapter trait by delegating to a
// simple deterministic reply (used by higher-level async flows/tests).
use super::trait_adapter::{LLMAdapter, LLMRequest, LLMResponse};
use async_trait::async_trait;

pub struct LLMMockAdapter {
    pub reply_prefix: String,
}

impl LLMMockAdapter {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self { reply_prefix: prefix.into() }
    }
}

#[async_trait]
impl LLMAdapter for LLMMockAdapter {
    async fn call(&self, req: LLMRequest) -> Result<LLMResponse, String> {
        Ok(LLMResponse { text: format!("{}{}", self.reply_prefix, req.prompt) })
    }

    fn name(&self) -> &'static str {
        "mock-llm"
    }
}

impl MockAdapter {
    // new() will check MOCK_ADAPTER_RESPONSE env var. If set and points to a file
    // containing a JSON array or object, it will use that as the deterministic response.
    pub fn new() -> Self {
        let seeded = env::var("MOCK_ADAPTER_RESPONSE").ok().and_then(|p| {
            // try treat as file path first
            if fs::metadata(&p).is_ok() {
                let data = fs::read_to_string(&p).ok()?;
                serde_json::from_str(&data).ok()
            } else {
                // try parse the string itself as JSON
                serde_json::from_str(&p).ok()
            }
        });
        // determine optional state file from CPRCODR_OUTPUT env
        let state_path = env::var("CPRCODR_OUTPUT")
            .ok()
            .map(|d| PathBuf::from(d).join("mock_state.json"));
        // try read initial call count if present
        let initial_count = if let Some(p) = &state_path {
            if p.exists() {
                fs::read_to_string(p)
                    .ok()
                    .and_then(|s| s.trim().parse::<usize>().ok())
                    .unwrap_or(0)
            } else {
                0
            }
        } else {
            0
        };

        MockAdapter {
            seeded,
            seq_index: Mutex::new(0),
            call_count: Mutex::new(initial_count),
            state_path,
        }
    }

    // helper to create with explicit seeded value (used by unit tests)
    pub fn with_seed(value: Value) -> Self {
        MockAdapter {
            seeded: Some(value),
            seq_index: Mutex::new(0),
            call_count: Mutex::new(0),
            state_path: None,
        }
    }
}

impl Backend for MockAdapter {
    fn generate(&self, prompt: &str, options: Value) -> Result<Artifacts, BackendError> {
        // respect dry_run option (do not advance or persist mock state when true)
        let dry_run = options
            .get("dry_run")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if let Some(seed_val) = &self.seeded {
            // If top-level is an array, we need to decide if it is:
            //  - an array of artifact objects (each with `path`/`summary`) -> return as artifacts
            //  - an array of seed objects -> return sequentially across calls
            let seed = if seed_val.is_array() {
                let arr = seed_val.as_array().unwrap();
                // if the first element looks like an artifact (has 'path' or 'summary'),
                // treat the whole array as the artifacts response
                if let Some(first) = arr.get(0) {
                    if first.is_object()
                        && (first.get("path").is_some()
                            || first.get("summary").is_some()
                            || first.get("content").is_some())
                    {
                        // return the entire artifacts array directly
                        return Ok(Artifacts {
                            artifacts: arr.clone(),
                        });
                    }
                }
                // otherwise treat as sequential seeds
                let mut idx = self.seq_index.lock().unwrap();
                let el = arr
                    .get(*idx % arr.len())
                    .cloned()
                    .unwrap_or_else(|| json!({}));
                *idx = idx.wrapping_add(1);
                el
            } else {
                seed_val.clone()
            };

            // compute and possibly persist call count for intermittent behavior
            let mut call_ct = self.call_count.lock().unwrap();
            let current = *call_ct;
            let effective = if dry_run {
                current
            } else {
                current.wrapping_add(1)
            };
            if !dry_run {
                *call_ct = effective;
                // persist call count if requested
                if let Some(p) = &self.state_path {
                    let parent = p.parent().unwrap_or_else(|| std::path::Path::new("."));
                    let _ = fs::create_dir_all(parent);
                    let _ = fs::write(p, call_ct.to_string());
                }
            }

            // If seeded is an object with `artifacts` key, use it; otherwise if it's an array use that
            // If seeded looks like an artifact object (has "path"), wrap it into an array
            if seed.is_object() && seed.get("artifacts").is_some() {
                // check intermittent configuration: if present and instructs failure on this call
                if let Some(inter) = seed.get("intermittent") {
                    if let Some(fail_on) = inter.get("fail_on_call").and_then(|v| v.as_u64()) {
                        if (effective as u64) == fail_on {
                            if let Some(kind) = inter.get("kind").and_then(|k| k.as_str()) {
                                let msg = inter
                                    .get("message")
                                    .and_then(|m| m.as_str())
                                    .unwrap_or("intermittent failure");
                                match kind {
                                    "network" => {
                                        return Err(BackendError::Network(msg.to_string()))
                                    }
                                    _ => return Err(BackendError::Protocol(msg.to_string())),
                                }
                            }
                        }
                    }
                }
                // simulate delay if requested
                if let Some(d) = seed.get("timeout_ms").and_then(|v| v.as_u64()) {
                    let ms = d.min(10_000);
                    thread::sleep(Duration::from_millis(ms));
                }
                // partial flag: drop some fields
                if seed
                    .get("partial")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
                {
                    let arr = seed["artifacts"].as_array().cloned().unwrap_or_default();
                    let partial: Vec<_> = arr
                        .into_iter()
                        .map(|mut item| {
                            if let Some(obj) = item.as_object_mut() {
                                obj.remove("content");
                            }
                            item
                        })
                        .collect();
                    return Ok(Artifacts { artifacts: partial });
                }

                let arr = seed["artifacts"].as_array().cloned().unwrap_or_default();
                return Ok(Artifacts { artifacts: arr });
            }
            // single artifact object shaped like { path, summary, content }
            if seed.is_object() && seed.get("path").is_some() {
                return Ok(Artifacts {
                    artifacts: vec![seed],
                });
            }
            // if seeded object contains error field, return protocol error
            if seed.is_object() && seed.get("error").is_some() {
                let msg = seed["error"].as_str().unwrap_or("mock error");
                return Err(BackendError::Protocol(msg.to_string()));
            }
            // if seeded object contains network field, return network error
            if seed.is_object() && seed.get("network").is_some() {
                let msg = seed["network"].as_str().unwrap_or("mock network error");
                return Err(BackendError::Network(msg.to_string()));
            }
            if seed.is_array() {
                return Ok(Artifacts {
                    artifacts: seed.as_array().cloned().unwrap_or_default(),
                });
            }
        }

        // fallback deterministic mock response: return a single artifact with path and summary
        let art = json!({
            "path": "generated/mock.txt",
            "summary": format!("mock-generated for prompt: {}", prompt),
            "content": format!("This is a mock artifact for prompt: {}", prompt)
        });
        Ok(Artifacts {
            artifacts: vec![art],
        })
    }
}
