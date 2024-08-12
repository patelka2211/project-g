mod helpers;

use crate::shell::git::run_git_command;
use helpers::collect_branches;
use serde::Serialize;
use std::path::Path;

#[tauri::command]
pub fn local_branches(repo_path: String) -> core::result::Result<String, String> {
    let heads = format!("{}/.git/refs/heads", repo_path);
    let heads = Path::new(&heads);
    let branches = collect_branches(&heads, &None);

    match branches {
        Ok(branches) => match serde_json::to_string(&branches) {
            Ok(branches) => Ok(branches),
            Err(_) => Err("Error while stringifying branches.".into()),
        },
        Err(_) => Err("Error while collecting branches.".into()),
    }
}

#[derive(Serialize)]
struct BranchInfo {
    #[serde(rename(serialize = "pointsAt"))]
    points_at: String,
    #[serde(rename(serialize = "updatedAt"))]
    updated_at: String,
    msg: String,
}

#[tauri::command]
pub fn get_branch_info(
    repo_path: String,
    branch_name: String,
) -> core::result::Result<String, String> {
    let output = match run_git_command(
        &repo_path,
        &format!("log -1 --format=%H$:$%cI$:$%B refs/heads/{}", branch_name),
    ) {
        Ok(output) => output,
        Err(error_msg) => return Err(error_msg.to_string().into()),
    };

    let output: Vec<&str> = output.split("$:$").collect();

    let points_at = (match output.get(0) {
        Some(value) => value,
        None => return Err("Error reading branch commit hash.".into()),
    })
    .to_string();

    let updated_at = (match output.get(1) {
        Some(value) => value,
        None => return Err("Error reading commit time.".into()),
    })
    .to_string();

    let msg = (match output.get(2) {
        Some(value) => value,
        None => return Err("Error reading commit message.".into()),
    })
    .to_string();

    let output = match serde_json::to_string(&BranchInfo {
        points_at,
        updated_at,
        msg,
    }) {
        Ok(output) => output,
        Err(error) => return Err(error.to_string()),
    };

    Ok(output)
}

#[tauri::command]
pub fn current_branch(repo_path: String) -> core::result::Result<String, String> {
    let output = match run_git_command(&repo_path, &"branch --show-current".to_string()) {
        Ok(output) => output,
        Err(error) => return Err(error.to_string()),
    };

    if output == "" {
        return Err("Current branch not found. Probably the HEAD is detached.".into());
    }

    Ok(output)
}
