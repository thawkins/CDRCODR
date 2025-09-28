use cprcodr_core::adapters::MockAdapter;
use cprcodr_core::Backend;
use cprcodr_core::BackendError;
use serde_json::json;

#[test]
fn mock_adapter_returns_error_when_seeded() {
    let seed = json!({ "error": "simulated failure" });
    let a = MockAdapter::with_seed(seed);
    let res = a.generate("prompt", json!({}));
    assert!(res.is_err(), "expected error from seeded mock");
}

#[test]
fn mock_adapter_sequential_and_partial() {
    // sequential seeds: first returns artifacts with content, second returns partial (no content)
    let seed = json!([
        { "artifacts": [{ "path": "a.txt", "summary": "one", "content": "c1" }] },
        { "artifacts": [{ "path": "b.txt", "summary": "two", "content": "c2" }], "partial": true }
    ]);
    let a = MockAdapter::with_seed(seed);
    let r1 = a.generate("p", json!({})).expect("first ok");
    assert_eq!(r1.artifacts.len(), 1);
    assert!(r1.artifacts[0].get("content").is_some());

    let r2 = a.generate("p", json!({})).expect("second ok");
    assert_eq!(r2.artifacts.len(), 1);
    assert!(
        r2.artifacts[0].get("content").is_none(),
        "expected partial without content"
    );
}

#[test]
fn mock_adapter_timeout_respects_delay() {
    let seed =
        json!({ "artifacts": [{ "path": "t.txt", "summary": "delayed" }], "timeout_ms": 50 });
    let a = MockAdapter::with_seed(seed);
    let before = std::time::Instant::now();
    let _ = a.generate("p", json!({})).expect("ok");
    let elapsed = before.elapsed();
    assert!(elapsed.as_millis() >= 50, "expected at least 50ms delay");
}

#[test]
fn mock_adapter_network_error() {
    let seed = json!({ "network": "connection refused" });
    let a = MockAdapter::with_seed(seed);
    let res = a.generate("prompt", json!({}));
    match res {
        Err(BackendError::Network(msg)) => {
            assert!(msg.contains("connection refused"));
        }
        other => panic!("expected network error, got: {:?}", other),
    }
}

#[test]
fn mock_adapter_intermittent_failure() {
    let seed = json!({
        "artifacts": [{ "path": "a.txt", "summary": "ok", "content": "c1" }],
        "intermittent": { "fail_on_call": 2, "kind": "network", "message": "boom" }
    });
    let a = MockAdapter::with_seed(seed);
    // first call should succeed
    let r1 = a.generate("p", json!({})).expect("first ok");
    assert_eq!(r1.artifacts.len(), 1);
    // second call should fail with network error
    let r2 = a.generate("p", json!({}));
    match r2 {
        Err(BackendError::Network(msg)) => assert!(msg.contains("boom")),
        other => panic!("expected network error, got: {:?}", other),
    }
}
