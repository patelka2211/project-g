use std::process::{Command, Output};

use crate::error::Result;

pub fn run_command(repo_path: &String, command: &str, args: Option<&Vec<&str>>) -> Result<String> {
    let mut git_args = vec![command];

    if let Some(args) = args {
        git_args.extend(args);
    }

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
        let stderr = stderr
            .strip_suffix("\n")
            .map(|x| x.to_string())
            .unwrap_or(stderr);

        return Err(stderr.into());
    }

    let stdout = String::from_utf8(stdout)?;
    let stdout = stdout
        .strip_suffix("\n")
        .map(|x| x.to_string())
        .unwrap_or(stdout);

    Ok(stdout)
}
