use std::path::PathBuf;
use crate::prelude::WorkflowResult;
use crate::workflow::file_format::Workflow;
use crate::workflow::repository::directory::DirectoryRepository;
use crate::workflow::repository::WorkflowRepository;

struct GitRepository {
    url: String,
    branch: String,
    root: PathBuf,
    directoryRepository: DirectoryRepository,
}

use git2::Repository;

impl GitRepository {

    pub fn new(url: String, branch: String, path_buf: PathBuf) -> Self {
        Self::clone(&url, path_buf.clone());
        GitRepository {
            url,
            branch,
            root: path_buf.clone(),
            directoryRepository: DirectoryRepository::new(path_buf),
        }
    }

    fn clone(url: &str, root: PathBuf) -> WorkflowResult<()> {
        // clone the git repository
        let repo = Repository::clone(url, root);
        if let Err(e) = repo {
            panic!("Error cloning git repository: {}", e)
        }
        Ok(())
    }

}

impl WorkflowRepository for GitRepository {
    fn refresh(&mut self) -> WorkflowResult<()> {
        // pull from git
        // update directoryRepository
        self.directoryRepository.refresh()
    }

    fn get_workflow(&self, name: impl Into<String>) -> WorkflowResult<Workflow> {
        self.directoryRepository.get_workflow(name)
    }

    fn get_workflows(&self) -> WorkflowResult<Vec<Workflow>> {
        self.directoryRepository.get_workflows()
    }

    fn save_workflow(&mut self, workflow: Workflow) -> WorkflowResult<()> {
        self.directoryRepository.save_workflow(workflow)
    }

    fn delete_workflow(&mut self, name: &str) -> WorkflowResult<()> {
        self.directoryRepository.delete_workflow(name)
    }

    fn query_workflows(&self, query: &str) -> WorkflowResult<Vec<Workflow>> {
        self.directoryRepository.query_workflows(query)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::workflow::repository::git::GitRepository;

    #[test]
    fn test_load_from_git() {
        let repo = GitRepository::new("https://github.com/warpdotdev/workflows.git".to_string(), "main".to_string(), PathBuf::from("tests/fixtures/github/warpdotdev"));
    }
}