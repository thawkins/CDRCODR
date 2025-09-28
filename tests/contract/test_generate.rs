use cprcodr_core::adapters::MockAdapter;
use serde_json::json;

// Contract test replaced with an offline check using MockAdapter so CI doesn't
// rely on external services. This asserts the generated artifacts shape.
#[test]
fn contract_generate_returns_artifacts_shape() {
    let adapter = MockAdapter::new();
    let res = adapter.generate("Create a hello world Rust project", json!({}));
    assert!(res.is_ok(), "MockAdapter should produce a response");
    let arts = res.unwrap().artifacts;
    assert!(!arts.is_empty(), "Artifacts array must not be empty");
    // check artifact has expected fields
    let first = &arts[0];
    assert!(first.get("path").is_some(), "artifact must include path");
    assert!(
        first.get("summary").is_some(),
        "artifact must include summary"
    );
}
