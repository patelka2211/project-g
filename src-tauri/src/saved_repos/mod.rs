use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct RepoInfo {
    name: String,
    dir: String,
}

fn get_store_file_path() -> &'static Path {
    Path::new("/Users/kp/Documents/GitHub/kpverse/project-g/dev-data/saved-repos.json")
}

fn get_store_content() -> Result<String> {
    let path = get_store_file_path();

    if path.exists() {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(content)
    } else {
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        let default_value = "{}";
        file.write_all(default_value.as_bytes())?;

        Ok(default_value.to_string())
    }
}

fn save_to_store(content: &String) -> Result<()> {
    let path = get_store_file_path();

    let mut file = OpenOptions::new()
        .write(true)
        .create(!path.exists())
        .open(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn get_repos_helper() -> Result<HashMap<String, RepoInfo>> {
    let saved_repos_file = get_store_content()?;
    Ok(from_str(&saved_repos_file)?)
}

#[tauri::command]
pub fn get_repos() -> core::result::Result<HashMap<String, RepoInfo>, String> {
    match get_repos_helper() {
        Ok(repos) => Ok(repos),
        Err(error) => Err(error.to_string()),
    }
}

fn add_repo_helper(dir: String, name: String) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let mut repos = get_repos()?;

    for (id, repo) in repos.iter() {
        if repo.dir == dir && repo.name == name {
            return Ok(id.to_owned());
        }
    }

    repos.insert(id.clone(), RepoInfo { dir, name });
    save_to_store(&to_string(&repos)?)?;

    Ok(id)
}
#[tauri::command]
pub fn add_repo(dir: String, name: String) -> core::result::Result<String, String> {
    match add_repo_helper(dir, name) {
        Ok(id) => Ok(id),
        Err(error) => Err(error.to_string()),
    }
}

fn delete_repo_helper(id: String) -> Result<()> {
    let mut repos = get_repos()?;
    repos.remove(&id);
    save_to_store(&to_string(&repos)?)?;
    Ok(())
}

#[tauri::command]
pub fn delete_repo(id: String) -> core::result::Result<(), String> {
    match delete_repo_helper(id) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
