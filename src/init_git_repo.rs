use std::process::Command;

pub(crate) fn init_git_repo() {
    let output = Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to initialize git repository");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
