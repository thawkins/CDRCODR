use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn gen_with_mock_backend_saves_artifacts() {
    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    // run gen with mock backend
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .arg("gen")
        .arg("hello world")
        .arg("--backend")
        .arg("mock");
    cmd.assert().success();

    // find the artifacts file (session id unknown) - read directory
    let entries: Vec<_> = std::fs::read_dir(out_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    // expect at least one JSON file with -artifacts.json or session file
    assert!(!entries.is_empty(), "expected files in output dir");
}
