use std::process::Command;

pub(crate) fn has_unstaged_changes() -> bool {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect("Failed to run git status");

    let status = String::from_utf8_lossy(&output.stdout);
    !status.trim().is_empty()
}
