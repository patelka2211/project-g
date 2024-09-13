use serde::Deserialize;

use crate::_backend_specific::git::utilities::SerializableBranchType;

mod utilities {

    use git2::{BranchType, Repository};
    use serde::Serialize;

    use crate::{_backend_specific::git::utilities::SerializableBranchType, error::Result};

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
    }

    #[derive(Serialize)]
    pub struct ParentCommits {
        list: Vec<CommitInfo>,
        #[serde(rename(serialize = "endOfCommits"))]
        end_of_commits: bool,
    }

    pub fn get_parent_commits(
        repo_path: &String,
        branch_data: &BranchData,
        no_of_commits: &u8,
    ) -> Result<ParentCommits> {
        let repo = Repository::open(repo_path)?;

        let mut commit = Some(
            repo.find_branch(
                &branch_data.name,
                match branch_data.branch_type {
                    SerializableBranchType::Local => BranchType::Local,
                    SerializableBranchType::Remote => BranchType::Remote,
                },
            )?
            .get()
            .peel_to_commit()?,
        );

        let mut list: Vec<CommitInfo> = Vec::new();

        let mut end_of_commits = false;

        for _index in 0u8..*no_of_commits {
            match commit {
                Some(some_commit) => {
                    // commit hash
                    let hash = some_commit.id().to_string();

                    // commit message
                    let msg = some_commit.message().unwrap_or("").to_string();

                    // commit's author's signature
                    let signature = some_commit.author();
                    // commit's author's name
                    let name = signature.name().map(|e| e.to_string());
                    // commit's author's email
                    let email = signature.email().map(|e| e.to_string());

                    list.push(CommitInfo {
                        hash,
                        msg,
                        author: AuthorInfo { name, email },
                    });

                    let parent_0 = match some_commit.parent(0) {
                        Ok(commit) => Some(commit),
                        Err(_) => None,
                    };

                    commit = parent_0;
                }
                None => {
                    end_of_commits = true;
                    break;
                }
            };
        }

        Ok(ParentCommits {
            list,
            end_of_commits,
        })
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
) -> Result<utilities::ParentCommits, String> {
    match utilities::get_parent_commits(&repo_path, &branch_data, &no_of_commits) {
        Ok(output) => Ok(output),
        Err(error) => Err(error.to_string()),
    }
}
