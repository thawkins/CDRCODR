use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn dry_run_does_not_advance_mock_state() {
    let td = tempdir().unwrap();
    let out_dir = td.path();

    let seed = json!({
        "artifacts": [{ "path": "d.txt", "summary": "ok", "content": "dc" }],
        "intermittent": { "fail_on_call": 2, "kind": "network", "message": "boom" }
    });
    let seeded = serde_json::to_string(&seed).unwrap();

    // First, run with --dry-run; it should succeed and not create/persist mock_state.json
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir.to_str().unwrap())
        .env("MOCK_ADAPTER_RESPONSE", seeded.clone())
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock")
        .arg("--dry-run");
    cmd.assert().success();

    let state = out_dir.join("mock_state.json");
    assert!(!state.exists(), "dry-run should not persist mock state");

    // Now run normally (no dry-run) — first non-dry run should succeed (call 1)
    let mut cmd2 = Command::cargo_bin("cprcodr").unwrap();
    cmd2.env("CPRCODR_OUTPUT", out_dir.to_str().unwrap())
        .env("MOCK_ADAPTER_RESPONSE", seeded.clone())
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock");
    cmd2.assert().success();

    // Now the second normal run should fail due to intermittent network error (call 2)
    let mut cmd3 = Command::cargo_bin("cprcodr").unwrap();
    cmd3.env("CPRCODR_OUTPUT", out_dir.to_str().unwrap())
        .env("MOCK_ADAPTER_RESPONSE", seeded)
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock");
    cmd3.assert()
        .failure()
        .stderr(predicate::str::contains("network error"));
}
