use assert_cmd::Command;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn apply_falls_back_to_summary_when_content_missing() {
    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    let session_id = "test-content-fallback";
    let artifact = json!({
        "path": "fallback.txt",
        "summary": "SUMMARY ONLY"
    });
    let arr = json!([artifact]);

    let mut artifacts_path = td.path().to_path_buf();
    artifacts_path.push(format!("{}-artifacts.json", session_id));
    std::fs::write(&artifacts_path, serde_json::to_string_pretty(&arr).unwrap())
        .expect("write artifacts");

    // sanity-check artifacts file exists where we wrote it
    assert!(
        artifacts_path.exists(),
        "artifacts file should exist at {:?}",
        artifacts_path
    );

    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .current_dir(td.path())
        .arg("--output")
        .arg(out_dir)
        .arg("apply")
        .arg(session_id);
    cmd.assert().success();

    let mut artifact_path = td.path().to_path_buf();
    artifact_path.push("fallback.txt");
    let written = std::fs::read_to_string(&artifact_path).expect("read written artifact");
    assert!(
        written.contains("SUMMARY ONLY"),
        "expected written file to contain summary when content missing"
    );
}
