use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn mock_reset_removes_state_file() {
    let td = tempdir().unwrap();
    let out_dir = td.path();
    let state = out_dir.join("mock_state.json");
    fs::write(&state, "123").unwrap();

    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir.to_str().unwrap())
        .arg("mock-reset");
    cmd.assert().success();
    assert!(!state.exists(), "mock_state.json should be removed");
}
