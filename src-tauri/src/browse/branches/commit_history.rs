use crate::_backend_specific::git::executor::run_command;

mod utilities {

    use git2::{Oid, Repository};
    use serde::Serialize;

    use crate::error::Result;

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
        commit_hash: &String,
        no_of_commits: &u8,
    ) -> Result<ParentCommits> {
        let repo = Repository::open(repo_path)?;

        let mut commit = Some(repo.find_commit(Oid::from_str(commit_hash)?)?);

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

#[tauri::command]
pub fn get_parent_commits(
    repo_path: String,
    commit_hash: String,
    no_of_commits: u8,
) -> Result<utilities::ParentCommits, String> {
    match utilities::get_parent_commits(&repo_path, &commit_hash, &no_of_commits) {
        Ok(output) => Ok(output),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn cherry_pick_commit(repo_path: String, commit_hash: String) -> Result<String, String> {
    match run_command(&repo_path, "cherry-pick", &vec![&commit_hash]) {
        Ok(output) => Ok(output),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn revert_commit(repo_path: String, commit_hash: String) -> Result<String, String> {
    match run_command(&repo_path, "revert", &vec![&commit_hash]) {
        Ok(output) => Ok(output),
        Err(error) => Err(error.to_string()),
    }
}
