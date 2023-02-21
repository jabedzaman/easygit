use std::process::{Command, Stdio};

pub(crate) fn stash_changes() {
    let output = Command::new("git")
        .arg("stash")
        .arg("--include-untracked")
        .arg("--message")
        .arg("rust-git stash")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to stash changes");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
