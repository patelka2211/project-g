use std::process::{Command, Output};

use crate::error::Result;

pub fn run(repo_path: &String, command: &str, args: &Vec<&str>) -> Result<String> {
    let mut git_args = vec![command];
    git_args.extend(args);

    let Output {
        status,
        stdout,
        stderr,
    } = Command::new("git")
        .args(git_args)
        .current_dir(repo_path)
        .output()?;

    if !status.success() {
        let stderr = String::from_utf8(stderr)?;
        let stderr = match stderr.strip_suffix("\n") {
            Some(stderr) => stderr.to_string(),
            None => stderr,
        };

        return Err(stderr.into());
    }

    let stdout = String::from_utf8(stdout)?;
    let stdout = match stdout.strip_suffix("\n") {
        Some(stdout) => stdout.to_string(),
        None => stdout,
    };

    Ok(stdout)
}
