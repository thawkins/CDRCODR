use serde_json::json;

#[test]
fn session_roundtrip_serialization_should_exist() {
    // Expects `Session` type to exist in `crates/core/src/session.rs` and
    // support serde (de)serialisation.
    let sample = json!({
        "id": "00000000-0000-0000-0000-000000000000",
        "created_at": "2025-09-28T00:00:00Z",
        "project_id": ".",
        "backend": "mock",
        "prompts": [],
        "artifacts": [],
        "call_log": []
    });

    let s = sample.to_string();
    let _parsed: Result<cprcodr_core::session::Session, _> = serde_json::from_str(&s);
    assert!(
        _parsed.is_ok(),
        "Session (de)serialization must be implemented"
    );
}
