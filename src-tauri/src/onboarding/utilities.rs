use crate::error::Result;
use git2::Repository;
use serde::Serialize;

#[derive(Serialize)]
pub struct RemoteOrigin {
    fetch: Option<String>,
    push: Option<String>,
}

pub fn get_remote_origin(repo_path: String) -> Result<RemoteOrigin> {
    let repo = Repository::open(repo_path)?;

    let must_have_remote = "origin";

    let remote_origin = repo.find_remote(&must_have_remote)?;

    let fetch = match remote_origin.url() {
        Some(url) => Some(url.to_string()),
        None => None,
    };

    let push = match remote_origin.pushurl() {
        Some(url) => Some(url.to_string()),
        None => None,
    };

    Ok(RemoteOrigin { fetch, push })
}
