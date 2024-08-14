use git2::{BranchType, Repository};
use serde::Serialize;

#[derive(Serialize)]
struct Branch {
    name: String,
    upstream: Option<String>,
    #[serde(rename(serialize = "isHead"))]
    is_head: bool,
    #[serde(rename(serialize = "commitTime"))]
    commit_time: u64,
}

#[tauri::command]
/// returns list of **LOCAL** branches with its metadata.
///```ts
/// // returns
/// Array<{
///     name: string,
///     upstream?: string,
///     isHead: boolean,
///     commitTime: number
/// }>
/// ```
pub fn local_branches(repo_path: String) -> core::result::Result<String, String> {
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

        let commit_time = match repo.find_reference(&format!("refs/heads/{}", name)) {
            Ok(reference) => match reference.peel_to_commit() {
                Ok(commit) => commit.time().seconds() as u64,
                Err(_) => continue,
            },
            Err(_) => continue,
        };

        output.push(Branch {
            name,
            upstream,
            is_head,
            commit_time,
        });
    }

    let output = match serde_json::to_string(&output) {
        Ok(output) => output,
        Err(error) => return Err(error.to_string()),
    };

    Ok(output)
}
