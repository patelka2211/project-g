use serde::Deserialize;

use crate::{git_executor::run, git_utilities::compare_branches};

use super::utilities;

#[derive(Deserialize)]
pub struct RemoteBranchInfo<'a> {
    remote: &'a str,
    name: &'a str,
}

#[derive(Deserialize)]
pub enum BranchType {
    Local,
    Remote,
}

#[tauri::command]
pub fn switch_branch(repo_path: String, branch_name: String) -> Result<(), String> {
    match run(&repo_path, "switch", &vec![&branch_name]) {
        Ok(_output) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn fetch_branch(repo_path: String, remote_branch: RemoteBranchInfo) -> Result<(), String> {
    match run(
        &repo_path,
        "fetch",
        &vec![remote_branch.remote, remote_branch.name],
    ) {
        Ok(_output) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn pull_branch(repo_path: String, remote_branch_name: RemoteBranchInfo) -> Result<(), String> {
    match run(
        &repo_path,
        "pull",
        &vec![remote_branch_name.remote, remote_branch_name.name],
    ) {
        Ok(_output) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn push_branch(
    repo_path: String,
    remote_branch: RemoteBranchInfo,
    branch_type: BranchType,
) -> Result<(), String> {
    let mut args = match branch_type {
        BranchType::Local => vec!["--set-upstream"],
        BranchType::Remote => vec![],
    };

    args.extend(vec![remote_branch.remote, remote_branch.name]);

    match run(&repo_path, "push", &args) {
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
