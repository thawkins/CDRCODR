use serde_json::json;

#[test]
fn patch_model_roundtrip_should_exist() {
    // This test expects a `Patch` model with serde (de)serialisation to exist.
    // It will fail until `crates/core/src/patch.rs` provides the `Patch` type.
    let sample = json!({
        "path": "src/lib.rs",
        "hunks": [{"start": 1, "end": 3, "content": "// new"}],
        "metadata": {"author": "tester"}
    });

    let s = sample.to_string();

    // Attempt to deserialize into the expected type (not implemented yet).
        let _parsed: Result<cprcodr_core::patch::Patch, _> = serde_json::from_str(&s);
    assert!(
        _parsed.is_ok(),
        "Patch model (de)serialisation must be implemented"
    );
}
