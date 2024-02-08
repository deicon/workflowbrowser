use std::path::PathBuf;

use crate::prelude::directory::DirectoryRepository;
use crate::prelude::git::GitRepository;

pub struct AppState {
    pub git_repo: GitRepository,
    pub dir_repo: DirectoryRepository,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(url: &str, branch: &str, repo_dir: PathBuf, dir_path: PathBuf) -> Self {
        AppState {
            git_repo: GitRepository::new(url, branch, repo_dir),
            dir_repo: DirectoryRepository::new(dir_path),
            should_quit: false,
        }
    }
}