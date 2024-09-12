use git2::{BranchType, Commit, Oid, Repository};
use serde::{Deserialize, Serialize};

use crate::error::Result;

use super::executor::run_command;

pub struct BranchComparision {
    pub ahead: i32,
    pub _behind: i32,
}

pub fn compare_branches(
    repo_path: &String,
    branch1: &String,
    branch2: &String,
) -> Result<BranchComparision> {
    let temp = format!("{}...{}", branch1, branch2);
    let output = run_command(
        repo_path,
        "rev-list",
        &vec!["--count", "--left-right", temp.as_str()],
    )?;

    let output: Vec<&str> = output.split_whitespace().collect();

    let ahead = output[0].parse::<i32>()?;
    let behind = output[1].parse::<i32>()?;

    Ok(BranchComparision {
        ahead,
        _behind: behind,
    })
}

type CommitParents<'a> = (Option<Commit<'a>>, Option<Commit<'a>>);

pub fn get_commit_parents<'a>(repo: &'a Repository, commit_id: &Oid) -> Result<CommitParents<'a>> {
    let commit = repo.find_commit(*commit_id)?;

    let mut parents = commit.parents();

    let parents = (parents.nth(0), parents.nth(1));

    Ok(parents)
}

#[derive(Serialize)]
pub enum CommitType {
    Merged,
    Normal,
    First,
}

pub fn get_commit_type(commit_parents: &CommitParents) -> CommitType {
    let (parent_0, parent_1) = commit_parents;

    if parent_0.is_some() && parent_1.is_none() {
        CommitType::Normal
    } else if parent_0.is_some() && parent_1.is_some() {
        CommitType::Merged
    } else {
        CommitType::First
    }
}

#[derive(Serialize, Deserialize)]
pub enum SerializableBranchType {
    Local,
    Remote,
}

pub fn get_commit_id_from_branch_name(
    repo: &Repository,
    branch_name: &String,
    branch_type: &SerializableBranchType,
) -> Result<Oid> {
    let branch = repo.find_branch(
        &branch_name,
        match branch_type {
            &SerializableBranchType::Local => BranchType::Local,
            SerializableBranchType::Remote => BranchType::Remote,
        },
    )?;

    let commit = branch.get().peel_to_commit()?;

    Ok(commit.id())
}
