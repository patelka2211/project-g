mod utilities {
    use crate::{_backend_specific::git::executor::run_command, error::Result};

    pub fn delete_branch(
        repo_path: &String,
        branch_name: &String,
        delete_mode: &String,
    ) -> Result<()> {
        let args = vec![branch_name.as_str(), delete_mode.as_str()];

        run_command(&repo_path, "branch", Some(&args))?;

        Ok(())
    }
}

use serde::Deserialize;

use crate::_backend_specific::git::{
    executor::run_command,
    utilities::{compare_branches, SerializableBranchType},
};

#[derive(Deserialize)]
pub struct RemoteBranchInfo<'a> {
    remote: &'a str,
    name: &'a str,
}

#[tauri::command]
pub fn switch_branch(repo_path: String, branch_name: String) -> Result<(), String> {
    match run_command(&repo_path, "switch", Some(&vec![&branch_name])) {
        Ok(_output) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn fetch_branch(repo_path: String, remote_branch: RemoteBranchInfo) -> Result<(), String> {
    match run_command(
        &repo_path,
        "fetch",
        Some(&vec![remote_branch.remote, remote_branch.name]),
    ) {
        Ok(_output) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn pull_branch(repo_path: String, remote_branch: RemoteBranchInfo) -> Result<(), String> {
    match run_command(
        &repo_path,
        "pull",
        Some(&vec![remote_branch.remote, remote_branch.name]),
    ) {
        Ok(_output) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn push_branch(
    repo_path: String,
    remote_branch: RemoteBranchInfo,
    branch_type: SerializableBranchType,
) -> Result<(), String> {
    let mut args = match branch_type {
        SerializableBranchType::Local => vec!["--set-upstream"],
        SerializableBranchType::Remote => vec![],
    };

    args.push(&remote_branch.remote);
    args.push(&remote_branch.name);

    match run_command(&repo_path, "push", Some(&args)) {
        Ok(_output) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn delete_branch(
    repo_path: String,
    local_branch: String,
    remote_branch: Option<String>,
    force: Option<bool>,
) -> Result<(), String> {
    if let Some(force) = force {
        if force == true {
            return match utilities::delete_branch(&repo_path, &local_branch, &"-D".to_string()) {
                Ok(_) => Ok(()),
                Err(error) => Err(error.to_string()),
            };
        }
    }

    let remote_branch = remote_branch.unwrap_or("origin/HEAD".to_string());

    let branch_comparision = match compare_branches(&repo_path, &local_branch, &remote_branch) {
        Ok(branch_comparision) => branch_comparision,
        Err(error) => return Err(error.to_string()),
    };

    if branch_comparision.ahead > 0 {
        let reason = format!(
            "Branch \"{}\" is {} commit(s) ahead of branch \"{}\".",
            local_branch, branch_comparision.ahead, remote_branch
        );
        return Err(reason);
    }

    match utilities::delete_branch(&repo_path, &local_branch, &"-d".to_string()) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
