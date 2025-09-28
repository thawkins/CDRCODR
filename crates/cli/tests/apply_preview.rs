use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn apply_preview_dry_run_then_apply_writes() {
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

    // run preview in the tempdir (should not write files)
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.current_dir(td.path())
        .env("CPRCODR_OUTPUT", out_dir)
        .arg("preview")
        .arg(session_id);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("src/lib.rs"));

    // preview should not create the files in the working dir
    assert!(!td.path().join("src/lib.rs").exists());
    assert!(!td.path().join("README.md").exists());

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
