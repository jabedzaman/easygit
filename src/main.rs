use std::env;
use std::io::{stdin, stdout, Write};
use std::process::Command;

use crate::has_unstaged_changes::has_unstaged_changes;
use crate::init_git_repo::init_git_repo;
use crate::is_git_repo::is_git_repo;
use crate::stash_changes::stash_changes;

mod has_unstaged_changes;
mod init_git_repo;
mod is_git_repo;
mod stash_changes;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: rust-git [init | add | commit | push]");
        return;
    }

    let command = &args[1];

    if !is_git_repo() {
        print!("Current directory is not a git repository. Do you want to create a new git repository? (y/n) ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        match input.trim() {
            "y" | "" => {
                init_git_repo();
            }
            _ => {
                println!("No git repository created. Exiting.");
                return;
            }
        }
    }

    if has_unstaged_changes() {
        print!("You have unstaged changes. Do you want to stash them? (y/n) ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        match input.trim() {
            "y" | "" => {
                stash_changes();
            }
            _ => {
                println!("Changes not stashed.");
            }
        }
    }

    match command.as_str() {
        "init" => {
            println!("This directory is already a git repository.");
        }
        "add" => {
            let output = Command::new("git")
                .arg("add")
                .arg(".")
                .output()
                .expect("Failed to add files to git repository");

            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        "commit" => {
            let commit_message = match args.get(2) {
                Some(message) => message.to_string(),
                None => {
                    print!("Enter commit message: ");
                    let _ = stdout().flush();
                    let mut input = String::new();
                    stdin()
                        .read_line(&mut input)
                        .expect("Failed to read user input");
                    input.trim().to_string()
                }
            };

            let output = Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(commit_message)
                .output()
                .expect("Failed to commit changes to git repository");

            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        "push" => {
            let output = Command::new("git")
                .arg("push")
                .output()
                .expect("Failed to push changes to git repository");

            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        _ => {
            println!("Invalid command: {}", command);
            println!("Usage: rust-git [init | add | commit | push]");
        }
    }
}
