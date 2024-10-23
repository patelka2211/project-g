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
    let branch_comparison_format = format!("{}...{}", branch1, branch2);
    let output = run_command(
        repo_path,
        "rev-list",
        Some(&vec![
            "--count",
            "--left-right",
            branch_comparison_format.as_str(),
        ]),
    )?;

    let output: Vec<&str> = output.split_whitespace().collect();

    let ahead = output[0].parse::<i32>()?;
    let behind = output[1].parse::<i32>()?;

    Ok(BranchComparision {
        ahead,
        _behind: behind,
    })
}

#[derive(Serialize, Deserialize)]
pub enum SerializableBranchType {
    Local,
    Remote,
}
