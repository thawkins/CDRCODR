use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn gen_dry_run_does_not_persist() {
    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    let seeded_value = json!([
        { "path": "src/lib.rs", "summary": "lib", "content": "pub fn hello() {}" }
    ]);
    let seeded = serde_json::to_string(&seeded_value).unwrap();

    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .env("MOCK_ADAPTER_RESPONSE", seeded)
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock")
        .arg("--dry-run");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Dry-run"));

    // output directory should be empty (no .json files)
    let entries: Vec<_> = std::fs::read_dir(out_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert!(entries.is_empty(), "expected no files created in dry-run");
}
