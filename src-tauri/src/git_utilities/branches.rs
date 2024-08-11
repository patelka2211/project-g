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
    points_at: String,
    updated_at: String,
}

#[tauri::command]
pub fn get_branch_info(
    repo_path: String,
    branch_name: String,
) -> core::result::Result<String, String> {
    let output = match run_git_command(
        &repo_path,
        &format!("log -1 --format=%H$%cI refs/heads/{}", branch_name),
    ) {
        Ok(output) => output.replace("\n", ""),
        Err(error_msg) => {
            return Err(error_msg.to_string().into());
        }
    };

    let output: Vec<&str> = output.split('$').collect();

    let points_at = (match output.get(0) {
        Some(value) => value,
        None => {
            return Err("Cannot found branch pointer.".into());
        }
    })
    .to_string();

    let updated_at = (match output.get(1) {
        Some(value) => value,
        None => {
            return Err("Cannot found last updated time.".into());
        }
    })
    .to_string();

    let output = match serde_json::to_string(&BranchInfo {
        points_at,
        updated_at,
    }) {
        Ok(output) => output,
        Err(error) => {
            return Err(error.to_string());
        }
    };

    Ok(output)
}
