use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn e2e_gen_preview_apply_with_mock() {
    // prepare a seeded artifacts array
    let seeded_value = json!([
        { "path": "src/lib.rs", "summary": "lib", "content": "pub fn hello() {}" },
        { "path": "README.md", "summary": "readme", "content": "# project" }
    ]);
    let seeded = serde_json::to_string(&seeded_value).unwrap();

    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    // run gen with mock backend and seeded response
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .env("MOCK_ADAPTER_RESPONSE", seeded)
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Saved session"));

    // find session artifacts file name
    let entries: Vec<_> = std::fs::read_dir(out_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    let artifacts_file = entries
        .iter()
        .find_map(|e| {
            let n = e.file_name().to_string_lossy().into_owned();
            if n.ends_with("-artifacts.json") {
                Some(e.path())
            } else {
                None
            }
        })
        .expect("artifacts file present");

    // preview the saved artifacts using the session id extracted from file name
    let fname = artifacts_file
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let session_id = fname.trim_end_matches("-artifacts.json");

    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .arg("preview")
        .arg(session_id);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("src/lib.rs"));

    // run apply which should write files into current workspace; run it in the tempdir
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.current_dir(td.path())
        .env("CPRCODR_OUTPUT", out_dir)
        .arg("apply")
        .arg(session_id);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Wrote artifact stub"));

    // verify files exist
    assert!(td.path().join("src/lib.rs").exists());
    assert!(td.path().join("README.md").exists());
}

#[test]
fn e2e_apply_with_git_commit() {
    let seeded_value = json!([
        { "path": "src/lib.rs", "summary": "lib", "content": "pub fn hello() {}" }
    ]);
    let seeded = serde_json::to_string(&seeded_value).unwrap();

    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    // seed and gen
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .env("MOCK_ADAPTER_RESPONSE", seeded)
        .arg("gen")
        .arg("generate project")
        .arg("--backend")
        .arg("mock");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Saved session"));

    // find artifacts file and session id
    let entries: Vec<_> = std::fs::read_dir(out_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    let artifacts_file = entries
        .iter()
        .find_map(|e| {
            let n = e.file_name().to_string_lossy().into_owned();
            if n.ends_with("-artifacts.json") {
                Some(e.path())
            } else {
                None
            }
        })
        .expect("artifacts file present");

    let fname = artifacts_file
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let session_id = fname.trim_end_matches("-artifacts.json");

    // run apply with git commit in the tempdir
    // initialize a git repo so apply --git-commit can create a branch and commit
    std::process::Command::new("git")
        .arg("init")
        .current_dir(td.path())
        .status()
        .expect("git init");
    // create an initial commit
    std::fs::write(td.path().join("README.md"), "initial").unwrap();
    std::process::Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(td.path())
        .status()
        .expect("git add");
    std::process::Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("initial")
        .current_dir(td.path())
        .status()
        .expect("git commit");

    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.current_dir(td.path())
        .env("CPRCODR_OUTPUT", out_dir)
        .arg("apply")
        .arg(session_id)
        .arg("--git-commit");
    cmd.assert().success();

    // verify the branch exists
    let out = std::process::Command::new("git")
        .arg("branch")
        .arg("--list")
        .current_dir(td.path())
        .output()
        .expect("git branch list");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("cprcodr-apply"));
}
