mod utilities {

    use chrono::{Datelike, Timelike, Utc};
    use git2::{Oid, Repository};

    use crate::error::Result;

    fn generate_branch_name() -> String {
        let now = Utc::now();

        format!(
            "branch-{:04}-{:02}-{:02}-{:02}-{:02}-{:02}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        )
    }

    pub fn create_branch(repo_path: String, start_point: String) -> Result<String> {
        let repo = Repository::open(&repo_path)?;
        let commit = repo.find_commit(Oid::from_str(&start_point)?)?;

        let new_branch = generate_branch_name();

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
