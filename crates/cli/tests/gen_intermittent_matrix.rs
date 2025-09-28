use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn cli_intermittent_protocol_fails_first_invocation() {
    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    let seed = json!({
        "artifacts": [{ "path": "p.txt", "summary": "ok", "content": "pc" }],
        "intermittent": { "fail_on_call": 1, "kind": "protocol", "message": "proto-bad" }
    });
    let seeded = serde_json::to_string(&seed).unwrap();

    // first run should fail immediately with protocol error
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .env("MOCK_ADAPTER_RESPONSE", seeded)
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("protocol error"));
}

#[test]
fn cli_intermittent_protocol_fail_on_three() {
    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    let seed = json!({
        "artifacts": [{ "path": "q.txt", "summary": "ok", "content": "qc" }],
        "intermittent": { "fail_on_call": 3, "kind": "protocol", "message": "proto-third" }
    });
    let seeded = serde_json::to_string(&seed).unwrap();

    // first two runs should succeed
    for _ in 0..2 {
        let mut cmd = Command::cargo_bin("cprcodr").unwrap();
        cmd.env("CPRCODR_OUTPUT", out_dir)
            .env("MOCK_ADAPTER_RESPONSE", seeded.clone())
            .arg("gen")
            .arg("generate project")
            .arg("--backend")
            .arg("mock");
        cmd.assert().success();
    }

    // third run should fail with protocol error
    let mut cmd3 = Command::cargo_bin("cprcodr").unwrap();
    cmd3.env("CPRCODR_OUTPUT", out_dir)
        .env("MOCK_ADAPTER_RESPONSE", seeded)
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock");
    cmd3.assert()
        .failure()
        .stderr(predicate::str::contains("protocol error"));
}

#[test]
fn cli_intermittent_network_fails_first_invocation() {
    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    let seed = json!({
        "artifacts": [{ "path": "n.txt", "summary": "ok", "content": "nc" }],
        "intermittent": { "fail_on_call": 1, "kind": "network", "message": "net-bad" }
    });
    let seeded = serde_json::to_string(&seed).unwrap();

    // first run should fail immediately with network error
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .env("MOCK_ADAPTER_RESPONSE", seeded)
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("network error"));
}
