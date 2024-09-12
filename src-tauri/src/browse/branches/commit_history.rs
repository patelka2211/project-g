use serde::Deserialize;

use crate::_backend_specific::git::utilities::SerializableBranchType;

mod utilities {

    use git2::Repository;
    use serde::Serialize;

    use crate::{
        _backend_specific::git::utilities::{
            get_commit_id_from_branch_name, get_commit_parents, get_commit_type, CommitType,
        },
        error::Result,
    };

    use super::BranchData;

    #[derive(Serialize)]
    struct AuthorInfo {
        name: Option<String>,
        email: Option<String>,
    }

    #[derive(Serialize)]
    pub struct CommitInfo {
        hash: String,
        msg: String,
        author: AuthorInfo,
        commit_type: CommitType,
    }

    pub fn get_parent_commits(
        repo_path: &String,
        branch_data: &BranchData,
        no_of_commits: &u8,
    ) -> Result<Vec<CommitInfo>> {
        let repo = Repository::open(repo_path)?;

        let mut commit_id =
            get_commit_id_from_branch_name(&repo, &branch_data.name, &branch_data.branch_type)?;

        let mut parents;

        let mut parents_list = Vec::new();

        for _index in 0u8..*no_of_commits {
            parents = match get_commit_parents(&repo, &commit_id) {
                Ok(parents) => parents,
                Err(_) => break,
            };

            let commit_type = get_commit_type(&parents);

            if let Some(parent_0) = parents.0 {
                // commit hash
                let parent_id = parent_0.id();

                // commit msg
                let msg = parent_0.message().unwrap_or("").to_string();

                // author's signature
                let signature = parent_0.author();
                let name = signature.name().map(|e| e.to_string());
                let email = signature.email().map(|e| e.to_string());

                parents_list.push(CommitInfo {
                    hash: parent_id.to_string(),
                    msg,
                    author: AuthorInfo { name, email },
                    commit_type,
                });

                commit_id = parent_id;
            }
        }

        Ok(parents_list)
    }
}

#[derive(Deserialize)]
pub struct BranchData {
    name: String,
    branch_type: SerializableBranchType,
}

#[tauri::command]
pub fn get_parent_commits(
    repo_path: String,
    branch_data: BranchData,
    no_of_commits: u8,
) -> Result<Vec<utilities::CommitInfo>, String> {
    match utilities::get_parent_commits(&repo_path, &branch_data, &no_of_commits) {
        Ok(output) => Ok(output),
        Err(error) => Err(error.to_string()),
    }
}
