use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn git_helper_creates_branch_and_commit() {
    // Expects a git helper API under `crates/core/src/git.rs` providing
    // `create_branch_and_commit(path, branch, message)` or similar. This test
    // will initialize a repo and call the helper.
    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    // Initialize git repo
    Command::new("git")
        .arg("init")
        .current_dir(dir_path)
        .assert()
        .success();

    // Create a dummy file and commit using the helper (not implemented yet)
    std::fs::write(dir_path.join("README.md"), "hello").unwrap();

    let res = cprcodr_core::git::create_branch_and_commit(dir_path, "cprcodr-apply", "test commit");
    assert!(res.is_ok(), "create_branch_and_commit must be implemented");
}
