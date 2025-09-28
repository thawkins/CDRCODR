use assert_cmd::Command;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn preview_prefers_content_over_summary() {
    let td = tempdir().unwrap();
    let out_dir = td.path().to_str().unwrap();

    // craft an artifacts file with both summary and content different
    let session_id = "test-content-priority";
    let artifact = json!({
        "path": "sample.txt",
        "summary": "SHORT SUMMARY",
        "content": "THE FULL CONTENT LINE 1\nLINE2\n"
    });
    let arr = json!([artifact]);

    // write artifacts file to output dir
    let mut artifacts_path = td.path().to_path_buf();
    artifacts_path.push(format!("{}-artifacts.json", session_id));
    std::fs::write(&artifacts_path, serde_json::to_string_pretty(&arr).unwrap())
        .expect("write artifacts");

    // run apply in the tempdir as the working directory so the artifact is written
    let mut cmd = Command::cargo_bin("cprcodr").unwrap();
    cmd.env("CPRCODR_OUTPUT", out_dir)
        .current_dir(td.path())
        .arg("apply")
        .arg(session_id);
    cmd.assert().success();

    // read the written artifact and ensure it contains the full content
    let mut artifact_path = td.path().to_path_buf();
    artifact_path.push("sample.txt");
    let written = std::fs::read_to_string(&artifact_path).expect("read written artifact");
    assert!(
        written.contains("THE FULL CONTENT LINE 1"),
        "expected written file to contain full content"
    );
}
