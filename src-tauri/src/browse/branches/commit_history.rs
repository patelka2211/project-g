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

    pub fn get_parent_commits(
        repo_path: &String,
        commit_hash: &String,
        no_of_commits: &u8,
    ) -> Result<Vec<CommitInfo>> {
        let repo = Repository::open(repo_path)?;

        let mut commit_id = Oid::from_str(&commit_hash)?;

        let mut parents_list = Vec::new();

        // let mut commit = repo.find_commit(Oid::from_str(&commit_hash)?)?;

        for _index in 0u8..*no_of_commits {
            let parent = match repo.find_commit(commit_id) {
                Ok(commit) => {
                    println!(
                        "{}'s no. of parent(s) is {}.",
                        commit.id(),
                        commit.parent_count()
                    );
                    match commit.parents().nth(0) {
                        Some(commit) => commit,
                        None => {
                            // println!("Commit {} not found.", commit_id);
                            break;
                        }
                    }
                }
                Err(_) => {
                    // println!("Commit {} not found.", commit_id);
                    break;
                }
            };
            // let parent = match commit.parents().nth(0) {
            //     Some(parent) => parent,
            //     None => break,
            // };

            // commit hash
            let parent_id = parent.id();
            // commit message
            let msg = parent.message().unwrap_or("").to_string();

            println!("{} {}", msg, parent_id);

            // author's signature
            let signature = parent.author();
            let name = match signature.name() {
                Some(name) => Some(name.to_string()),
                None => None,
            };
            let email = match signature.email() {
                Some(email) => Some(email.to_string()),
                None => None,
            };

            parents_list.push(CommitInfo {
                hash: parent_id.to_string(),
                msg,
                author: AuthorInfo { name, email },
            });

            commit_id = parent_id;
            // commit = match repo.find_commit(parent_id) {
            //     Ok(commit) => commit,
            //     Err(_) => break,
            // };
        }

        Ok(parents_list)
    }
}

#[tauri::command]
pub fn get_parent_commits(
    repo_path: String,
    commit_hash: String,
    no_of_commits: u8,
) -> Result<Vec<utilities::CommitInfo>, String> {
    match utilities::get_parent_commits(&repo_path, &commit_hash, &no_of_commits) {
        Ok(output) => Ok(output),
        Err(error) => Err(error.to_string()),
    }
}
