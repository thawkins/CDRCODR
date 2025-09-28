use std::path::Path;
use std::process::Command;

pub fn create_branch_and_commit(path: &Path, branch: &str, message: &str) -> Result<(), String> {
    // Minimal implementation using `git` commands. Real implementation should
    // handle errors and detect git availability.
    let res = Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch)
        .current_dir(path)
        .status()
        .map_err(|e| format!("git spawn failed: {}", e))?;

    if !res.success() {
        return Err("git checkout failed".into());
    }

    let res2 = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(path)
        .status()
        .map_err(|e| format!("git add failed: {}", e))?;

    if !res2.success() {
        return Err("git add failed".into());
    }

    let res3 = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .current_dir(path)
        .status()
        .map_err(|e| format!("git commit failed: {}", e))?;

    if !res3.success() {
        return Err("git commit failed".into());
    }

    Ok(())
}
