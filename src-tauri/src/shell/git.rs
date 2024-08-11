use crate::error::Result;
use std::process::{Command, Output};

pub fn run_git_command(repository: &String, command: &String) -> Result<String> {
    let command: Vec<&str> = command.trim().split_whitespace().collect();

    let Output {
        status,
        stdout,
        stderr,
    } = Command::new("git")
        .args(command)
        .current_dir(repository)
        .output()?;

    if status.success() {
        let stdout = String::from_utf8(stdout)?;

        return Ok(stdout);
    }

    let stderr = String::from_utf8(stderr)?;

    Err(stderr.into())
}
