use crate::prelude::{WorkflowError, WorkflowResult};
use crate::workflow::file_format::Workflow;
use crate::workflow::repository::directory::DirectoryRepository;
use crate::workflow::repository::WorkflowRepository;
use di::injectable;
use git2::build::{CheckoutBuilder, RepoBuilder};
use std::path::PathBuf;
#[injectable(WorkflowRepository)]
#[allow(dead_code)]
pub struct GitRepository {
    url: String,
    branch: String,
    root: PathBuf,
    directory_repository: DirectoryRepository,
}

impl GitRepository {
    #[doc = r"Create a new GitRepository"]
    pub fn new(url: &str, branch: &str, path_buf: PathBuf) -> Self {
        // clone git repository using https url
        if let Err(WorkflowError::IoError(message)) = Self::clone_repository(url, path_buf.clone())
        {
            println!("{}", message)
        };
        GitRepository {
            url: url.to_string(),
            branch: branch.to_string(),
            root: path_buf.clone(),
            directory_repository: DirectoryRepository::new(path_buf),
        }
    }

    fn clone_repository(url: &str, root: PathBuf) -> WorkflowResult<()> {
        // clone the git repository
        let mut builder = RepoBuilder::new();
        builder.with_checkout(CheckoutBuilder::new());

        let _ = builder
            .clone(url, root.as_path())
            .map_err(|_| WorkflowError::IoError("Unable to clone repo".to_string()));
        Ok(())
    }
}

impl WorkflowRepository for GitRepository {
    fn refresh(&mut self) -> WorkflowResult<()> {
        // pull from git
        // update directoryRepository
        self.directory_repository.refresh()
    }

    fn get_workflow(&self, name: &str) -> WorkflowResult<Workflow> {
        self.directory_repository.get_workflow(name)
    }

    fn get_workflows(&self) -> WorkflowResult<Vec<Workflow>> {
        self.directory_repository.get_workflows()
    }

    fn save_workflow(&mut self, workflow: Workflow) -> WorkflowResult<()> {
        self.directory_repository.save_workflow(workflow)
    }

    fn delete_workflow(&mut self, name: &str) -> WorkflowResult<()> {
        self.directory_repository.delete_workflow(name)
    }

    fn query_workflows(&self, query: &str) -> WorkflowResult<Vec<Workflow>> {
        self.directory_repository.query_workflows(query)
    }
}

#[cfg(test)]
mod tests {
    use crate::workflow::repository::git::GitRepository;
    use crate::workflow::repository::WorkflowRepository;
    use std::path::PathBuf;

    #[test]
    fn test_load_from_git() {
        let repo = GitRepository::new(
            "https://github.com/warpdotdev/workflows.git",
            "main",
            PathBuf::from("tests/fixtures/github/warpdotdev"),
        );
        assert_eq!(332, repo.get_workflows().unwrap().len());
    }
}
