use crate::{error::Result, git_executor::run};

pub fn delete_branch(repo_path: &String, branch_name: &String, delete_mode: &String) -> Result<()> {
    let args = vec!["branch", &branch_name, &delete_mode];

    run(&repo_path, "branch", &args)?;

    Ok(())
}
