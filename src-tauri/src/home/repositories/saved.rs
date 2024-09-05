mod utilities {
    use rusqlite::{params, Connection, Result as RusqliteResult};
    use serde::Serialize;
    use uuid::Uuid;

    use std::path::Path;

    use crate::error::Result;

    #[derive(Serialize)]
    pub struct RepoInfo {
        id: String,
        name: String,
        dir: String,
    }

    fn get_db_path() -> &'static Path {
        Path::new("/Users/kp/Documents/GitHub/kpverse/project-g/dev-data/metadata.db")
    }

    fn create_or_open_db() -> Result<Connection> {
        // Open a connection to the database
        let conn = Connection::open(get_db_path())?;

        // Create the saved_repos table if it does not exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS saved_repos (
                id   TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                dir  TEXT NOT NULL
            )",
            [],
        )?;

        Ok(conn)
    }

    // Function to add a new repo or return the ID if it already exists
    pub fn add_repo(name: String, dir: String) -> Result<String> {
        let conn = create_or_open_db()?;
        // Check if a repo with the same name and directory exists
        let mut stmt = conn.prepare("SELECT id FROM saved_repos WHERE name = ?1 AND dir = ?2")?;
        let existing_id: RusqliteResult<String> =
            stmt.query_row(params![name, dir], |row| row.get(0));

        if let Ok(id) = existing_id {
            return Ok(id);
        }

        // Create a new UUID if the repo doesn't exist
        let new_id = Uuid::new_v4().to_string();

        // Insert the new repo
        conn.execute(
            "INSERT INTO saved_repos (id, name, dir) VALUES (?1, ?2, ?3)",
            params![new_id, name, dir],
        )?;

        Ok(new_id)
    }

    // Function to list all repos in reverse order of insertion
    pub fn list_repos() -> Result<Vec<RepoInfo>> {
        let conn = create_or_open_db()?;
        let mut stmt = conn.prepare("SELECT id, name, dir FROM saved_repos ORDER BY rowid DESC")?;
        let repo_iter = stmt.query_map([], |row| {
            Ok(RepoInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                dir: row.get(2)?,
            })
        })?;

        let mut repos = Vec::new();
        for repo in repo_iter {
            repos.push(repo?);
        }
        Ok(repos)
    }

    // Function to delete a repo by id and return the deleted repo
    pub fn remove_repo(repo_id: String) -> Result<RepoInfo> {
        let conn = create_or_open_db()?;
        let mut stmt = conn.prepare("SELECT id, name, dir FROM saved_repos WHERE id = ?1")?;
        let repo = stmt.query_row(params![repo_id], |row| {
            Ok(RepoInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                dir: row.get(2)?,
            })
        })?;

        conn.execute("DELETE FROM saved_repos WHERE id = ?1", params![repo_id])?;

        Ok(repo)
    }

    // Function to update a repo list by deleting the old one and inserting the same as new entry
    pub fn reorder_repo(repo_id: String) -> Result<()> {
        let repo = remove_repo(repo_id)?;

        let conn = create_or_open_db()?;
        conn.execute(
            "INSERT INTO saved_repos (id, name, dir) VALUES (?1, ?2, ?3)",
            params![repo.id, repo.name, repo.dir],
        )?;

        Ok(())
    }
}

use utilities::RepoInfo;

#[tauri::command]
pub fn add_repo(name: String, dir: String) -> core::result::Result<String, String> {
    match utilities::add_repo(name, dir) {
        Ok(id) => Ok(id),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn list_repos() -> core::result::Result<Vec<RepoInfo>, String> {
    match utilities::list_repos() {
        Ok(repo_list) => Ok(repo_list),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn remove_repo(repo_id: String) -> core::result::Result<RepoInfo, String> {
    match utilities::remove_repo(repo_id) {
        Ok(deleted_repo) => Ok(deleted_repo),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn reorder_repo(repo_id: String) -> core::result::Result<(), String> {
    match utilities::reorder_repo(repo_id) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
