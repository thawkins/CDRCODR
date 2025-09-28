use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

// Failing integration test scaffold for T013: `cprcodr init` behavior

#[test]
fn init_creates_project_files() {
    let dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("cprcodr").expect("binary exists");
    cmd.current_dir(&dir.path()).arg("init");
    let assert = cmd.assert();
    // Expecting non-zero until CLI implemented
    assert.failure();
}
