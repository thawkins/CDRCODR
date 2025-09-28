use serde_json::json;

#[test]
fn artifact_metadata_roundtrip_should_exist() {
    // Expects `ArtifactMetadata` type in `crates/core/src/artifact.rs`.
    let sample = json!({
        "path": "src/lib.rs",
        "summary": "library",
        "checksum": null
    });

    let s = sample.to_string();
        let _parsed: Result<cprcodr_core::artifact::ArtifactMetadata, _> = serde_json::from_str(&s);
    assert!(
        _parsed.is_ok(),
        "ArtifactMetadata (de)serialization must be implemented"
    );
}
