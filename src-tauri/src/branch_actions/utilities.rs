use crate::{error::Result, git_executor::run};

pub fn delete_branch(repo_path: &String, branch_name: &String, delete_mode: &String) -> Result<()> {
    let args = vec![branch_name.as_str(), delete_mode.as_str()];

    run(&repo_path, "branch", &args)?;

    Ok(())
}
