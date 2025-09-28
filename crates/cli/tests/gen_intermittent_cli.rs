use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn cli_intermittent_failure_across_invocations() {
    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    let seed = json!({
        "artifacts": [{ "path": "a.txt", "summary": "ok", "content": "c1" }],
        "intermittent": { "fail_on_call": 2, "kind": "network", "message": "boom" }
    });
    let seeded = serde_json::to_string(&seed).unwrap();

    // first run should succeed
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .env("MOCK_ADAPTER_RESPONSE", seeded.clone())
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock");
    cmd.assert().success();

    // second run should fail due to intermittent state persisted
    let mut cmd2 = Command::cargo_bin("cprcodr").unwrap();
    cmd2.env("CPRCODR_OUTPUT", out_dir)
        .env("MOCK_ADAPTER_RESPONSE", seeded)
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock");
    cmd2.assert()
        .failure()
        .stderr(predicate::str::contains("network error"));
}
