use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn apply_writes_text_media_type() {
    let seeded_value = json!([
        { "path": "notes.txt", "summary": "notes", "content": "hello", "media_type": "text/plain" }
    ]);
    let seeded = serde_json::to_string(&seeded_value).unwrap();

    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

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

    // find session id
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

    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.current_dir(td.path())
        .env("CPRCODR_OUTPUT", out_dir)
        .arg("apply")
        .arg(session_id);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Wrote artifact stub"));

    assert!(td.path().join("notes.txt").exists());
}
