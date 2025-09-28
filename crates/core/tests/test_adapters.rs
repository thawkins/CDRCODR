use cprcodr_core::adapters::{LLMAdapter, LLMMockAdapter, OllamaAdapter};

#[tokio::test]
async fn test_ollama_adapter_name_and_call_error() {
    let a = OllamaAdapter::new("http://127.0.0.1:59999", None);
    assert_eq!(a.name(), "ollama");
    let req = cprcodr_core::adapters::trait_adapter::LLMRequest {
        prompt: "hi".into(),
        max_tokens: None,
    };
    // Expect network error or status error since nothing is listening on this port
    let res = a.call(req).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_llm_mock_adapter() {
    let m = LLMMockAdapter::new("prefix-");
    let req = cprcodr_core::adapters::trait_adapter::LLMRequest {
        prompt: "hello".into(),
        max_tokens: None,
    };
    let res = m.call(req).await.expect("mock should succeed");
    // adapters now return Vec<ArtifactMetadata>; expect one artifact with summary starting with prefix
    assert!(!res.is_empty());
    assert_eq!(res[0].summary, "prefix-hello");
}

#[tokio::test]
async fn test_lmstudio_adapter_name_and_call_error() {
    let a = cprcodr_core::adapters::LMStudioAdapter::new("http://127.0.0.1:59998", None);
    assert_eq!(a.name(), "lmstudio");
    let req = cprcodr_core::adapters::trait_adapter::LLMRequest {
        prompt: "hi".into(),
        max_tokens: None,
    };
    let res = a.call(req).await;
    assert!(res.is_err());
}
