use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn session_cli_flow() {
    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    // ensure no sessions initially
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .arg("session")
        .arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("no sessions found"));

    // simulate creating a session file
    let session_id = "00000000-0000-0000-0000-000000000001";
    let mut session_path = td.path().to_path_buf();
    session_path.push(format!("{}.json", session_id));
    fs::write(
        &session_path,
        r#"{ "id": "00000000-0000-0000-0000-000000000001", "project_id": "test" }"#,
    )
    .unwrap();

    // list should now show the file
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .arg("session")
        .arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(session_id));

    // show should print JSON
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .arg("session")
        .arg("show")
        .arg(session_id);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("project_id"));

    // rm should remove the files
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .arg("session")
        .arg("rm")
        .arg(session_id);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("removed session"));

    // list again should show no sessions
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .arg("session")
        .arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("no sessions found"));
}
