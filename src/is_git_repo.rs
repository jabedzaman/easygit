use std::process::Command;

pub(crate) fn is_git_repo() -> bool {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .expect("Failed to run git command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.trim() == "true"
}
