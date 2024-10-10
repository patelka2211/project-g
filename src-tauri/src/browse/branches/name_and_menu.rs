mod utilities {

    use git2::{BranchType::Local, Oid, Repository};

    use crate::error::Result;

    pub fn create_branch(repo_path: String, start_point: String) -> Result<String> {
        let repo = Repository::open(&repo_path)?;

        let count = repo.branches(Some(Local))?.count();

        let commit = repo.find_commit(Oid::from_str(&start_point)?)?;

        let new_branch = format!("untitled-branch-{}", count + 1);

        repo.branch(&new_branch, &commit, false)?;

        Ok(new_branch)
    }
}

#[tauri::command]
pub fn create_branch(repo_path: String, start_point: String) -> Result<String, String> {
    let new_branch = match utilities::create_branch(repo_path, start_point) {
        Ok(new_branch) => new_branch,
        Err(error) => return Err(error.to_string()),
    };

    Ok(new_branch)
}
