use crate::shell::git::run_git_command;
use git2::{BranchType, Repository};
use serde::Serialize;

#[derive(Serialize)]
struct Branch {
    name: String,
    upstream: Option<String>,
    #[serde(rename(serialize = "isHead"))]
    is_head: bool,
}

#[tauri::command]
pub fn list_local_branches(repo_path: String) -> core::result::Result<String, String> {
    let mut output: Vec<Branch> = Vec::new();

    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(error) => return Err(error.to_string()),
    };

    let branches = match repo.branches(Some(BranchType::Local)) {
        Ok(repo) => repo,
        Err(error) => return Err(error.to_string()),
    };

    for branch in branches {
        let branch = match branch {
            Ok(branch) => branch.0,
            Err(error) => return Err(error.to_string()),
        };

        let name = match branch.name() {
            Ok(name) => match name {
                Some(name) => name.to_string(),
                None => continue,
            },
            Err(_) => continue,
        };

        let upstream = match branch.upstream() {
            Ok(upstream_branch) => match upstream_branch.name() {
                Ok(name) => match name {
                    Some(name) => Some(name.to_string()),
                    None => continue,
                },
                Err(_) => None,
            },
            Err(_) => None,
        };

        let is_head = branch.is_head();

        output.push(Branch {
            name,
            upstream,
            is_head,
        });
    }

    let output = match serde_json::to_string(&output) {
        Ok(output) => output,
        Err(error) => return Err(error.to_string()),
    };

    Ok(output)
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
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(error) => return Err(error.to_string()),
    };

    let head = match repo.head() {
        Ok(head) => head,
        Err(error) => return Err(error.to_string()),
    };

    let name = head.name();

    match name {
        Some(name) => {
            let name = name.to_string();

            if name == String::from("HEAD") {
                return Err(String::from("Branch is detached."));
            }

            let name = name.replace("refs/heads/", "");

            Ok(name)
        }
        None => Err(String::from("Cannot read branch name.")),
    }
}
