use crate::error::Result;
use git2::{BranchType, Repository};
use serde::Serialize;

#[derive(Serialize)]
pub struct Branch {
    name: String,
    upstream: Option<String>,
    #[serde(rename(serialize = "isHead"))]
    is_head: bool,
    #[serde(rename(serialize = "commitTime"))]
    commit_time: u64,
}

pub fn get_local_branches(repo_path: String) -> Result<Vec<Branch>> {
    let mut output: Vec<Branch> = Vec::new();

    let repo = Repository::open(repo_path)?;

    let branches = repo.branches(Some(BranchType::Local))?;

    for branch in branches {
        let branch = branch?.0;

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

    Ok(output)
}
