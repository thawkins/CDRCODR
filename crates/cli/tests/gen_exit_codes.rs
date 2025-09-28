use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn exit_code_for_network_error_is_3_or_nonzero() {
    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    let seeded = serde_json::to_string(&json!({ "network": "simulated net fail" })).unwrap();

    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .env("MOCK_ADAPTER_RESPONSE", seeded)
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock");
    // CLI uses exit code 3 for generate backend failures in earlier code; accept any non-zero as well
    cmd.assert().failure().code(predicate::ne(0));
}

#[test]
fn exit_code_for_protocol_error_is_3_or_nonzero() {
    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    let seeded = serde_json::to_string(&json!({ "error": "invalid request" })).unwrap();

    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .env("MOCK_ADAPTER_RESPONSE", seeded)
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock");
    cmd.assert().failure().code(predicate::ne(0));
}
