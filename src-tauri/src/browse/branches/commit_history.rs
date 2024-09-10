mod utilities {
    use git2::{Oid, Repository};
    use serde::Serialize;

    use crate::error::Result;

    #[derive(Serialize)]
    struct CommitterInfo {
        name: Option<String>,
        email: Option<String>,
    }

    #[derive(Serialize)]
    pub struct CommitInfo {
        hash: String,
        msg: String,
        committer: CommitterInfo,
    }

    pub fn get_parent_commits(
        repo_path: &String,
        commit_hash: &String,
        no_of_commits: &u8,
    ) -> Result<Vec<CommitInfo>> {
        let repo = Repository::open(repo_path)?;
        let mut object_id = Oid::from_str(&commit_hash)?;

        let mut parents = Vec::new();

        for _index in 0u8..*no_of_commits {
            let parent = repo.find_commit(object_id)?.parent(0)?;

            // commit hash
            let hash = parent.id().to_string();
            // commit message
            let msg = parent.message().unwrap_or("").to_string();

            // committer's signature
            let signature = parent.committer();
            let name = match signature.name() {
                Some(name) => Some(name.to_string()),
                None => None,
            };
            let email = match signature.email() {
                Some(email) => Some(email.to_string()),
                None => None,
            };

            parents.push(CommitInfo {
                hash,
                msg,
                committer: CommitterInfo { name, email },
            });
            object_id = match parent.parent(0) {
                Ok(commit) => commit.id(),
                Err(_) => break,
            };
        }

        Ok(parents)
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
