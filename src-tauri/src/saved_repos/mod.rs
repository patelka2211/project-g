use crate::error::Result;
use serde::Deserialize;
use serde_json::from_str;
use std::{fs::File, io::Read};

#[derive(Deserialize)]
struct RepoInfo {
    name: String,
    id: String,
    dir: String,
}

fn list_repos() -> Result<Vec<RepoInfo>> {
    let path = "dev-data/saved-repos.json";
    let mut saved_repos_file = File::open(path)?;
    let mut saved_repos = String::new();

    saved_repos_file.read_to_string(&mut saved_repos)?;

    let repos: Vec<RepoInfo> = from_str(&saved_repos)?;

    Ok(repos)
}
