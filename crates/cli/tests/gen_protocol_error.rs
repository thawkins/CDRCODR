use assert_cmd::Command;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn gen_with_protocol_error_exits_nonzero() {
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
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("protocol error"));
}
