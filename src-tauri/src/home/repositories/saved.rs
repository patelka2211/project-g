mod utilities {
    use rusqlite::{params, Connection, Result as RusqliteResult};
    use serde::Serialize;
    use uuid::Uuid;

    use std::path::PathBuf;

    use crate::error::Result;

    #[derive(Serialize)]
    pub struct RepoInfo {
        id: String,
        name: String,
        dir: String,
    }

    fn create_or_open_db(db_path: &PathBuf) -> Result<Connection> {
        // Open a connection to the database
        let conn = Connection::open(db_path)?;

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
    pub fn add_repo(db_path: &PathBuf, name: &String, dir: &String) -> Result<String> {
        let conn = create_or_open_db(db_path)?;
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
    pub fn list_repos(db_path: &PathBuf) -> Result<Vec<RepoInfo>> {
        let conn = create_or_open_db(db_path)?;
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
    pub fn remove_repo(db_path: &PathBuf, repo_id: &String) -> Result<RepoInfo> {
        let conn = create_or_open_db(db_path)?;
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
    pub fn reorder_repo(db_path: &PathBuf, repo_id: &String) -> Result<()> {
        let repo = remove_repo(db_path, repo_id)?;
        let conn = create_or_open_db(db_path)?;

        conn.execute(
            "INSERT INTO saved_repos (id, name, dir) VALUES (?1, ?2, ?3)",
            params![repo.id, repo.name, repo.dir],
        )?;

        Ok(())
    }
}

use utilities::RepoInfo;

use std::{
    fs::{create_dir_all, OpenOptions},
    path::PathBuf,
};

use crate::error::Result;

fn app_data_dir_to_db_path(app_data_dir: &String) -> Result<PathBuf> {
    let db_path = PathBuf::from(app_data_dir).join("data.db");

    if db_path.exists() == true {
        return Ok(db_path);
    }

    let db_parent_path = match db_path.parent() {
        Some(path) => path,
        None => return Err("Cannot create directories for database.".into()),
    };

    create_dir_all(db_parent_path)?;

    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&db_path)?;

    Ok(db_path)
}

#[tauri::command]
pub fn add_repo(
    app_data_dir: String,
    name: String,
    dir: String,
) -> core::result::Result<String, String> {
    let db_path = match app_data_dir_to_db_path(&app_data_dir) {
        Ok(path) => path,
        Err(error) => return Err(error.to_string()),
    };

    match utilities::add_repo(&db_path, &name, &dir) {
        Ok(id) => Ok(id),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn list_repos(app_data_dir: String) -> core::result::Result<Vec<RepoInfo>, String> {
    let db_path = match app_data_dir_to_db_path(&app_data_dir) {
        Ok(path) => path,
        Err(error) => return Err(error.to_string()),
    };

    match utilities::list_repos(&db_path) {
        Ok(repo_list) => Ok(repo_list),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn remove_repo(
    app_data_dir: String,
    repo_id: String,
) -> core::result::Result<RepoInfo, String> {
    let db_path = match app_data_dir_to_db_path(&app_data_dir) {
        Ok(path) => path,
        Err(error) => return Err(error.to_string()),
    };

    match utilities::remove_repo(&db_path, &repo_id) {
        Ok(deleted_repo) => Ok(deleted_repo),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn reorder_repo(app_data_dir: String, repo_id: String) -> core::result::Result<(), String> {
    let db_path = match app_data_dir_to_db_path(&app_data_dir) {
        Ok(path) => path,
        Err(error) => return Err(error.to_string()),
    };

    match utilities::reorder_repo(&db_path, &repo_id) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
