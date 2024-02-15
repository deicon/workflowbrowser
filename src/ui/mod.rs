use std::path::PathBuf;
use crossterm::event::{Event, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::Rect;

use crate::prelude::directory::DirectoryRepository;
use crate::prelude::git::GitRepository;
use crate::prelude::{WorkflowError, WorkflowResult};
use crate::ui::components::command_list::CommandListComponent;
use crate::ui::components::{Component};
use crate::workflow::file_format::Workflow;
use crate::workflow::repository::WorkflowRepository;

pub mod components;

pub enum Focus {
    List,
}

pub struct AppState {
    pub git_repo: GitRepository,
    pub dir_repo: DirectoryRepository,
    pub focus: Focus,

    // components to be rendered
    pub command_list_component: Box<dyn Component>,
}

impl AppState {
    pub fn new(url: &str, branch: &str, repo_dir: PathBuf, dir_path: PathBuf) -> Box<Self> {
        Box::new(AppState {
            git_repo: GitRepository::new(url, branch, repo_dir),
            dir_repo: DirectoryRepository::new(dir_path),
            command_list_component: CommandListComponent::new(),
            focus: Focus::List,
        })
    }

}

impl WorkflowRepository for AppState {
    fn refresh(&mut self) -> WorkflowResult<()> {
        self.git_repo.refresh();
        self.dir_repo.refresh()

    }

    fn get_workflow(&self, name: &str) -> WorkflowResult<Workflow> {
        match self.dir_repo.get_workflow(name) {
            Ok(wkf) => { Ok(wkf) }
            Err(_) => {
                match self.git_repo.get_workflow(name) {
                    Ok(wkf) => { Ok(wkf) }
                    Err(_) => {Err(WorkflowError::NotFound(format!("{} not found", name)))}
                }
            }
        }
    }

    fn get_workflows(&self) -> WorkflowResult<Vec<Workflow>> {
        let mut all = self.dir_repo.get_workflows()?;
        let mut allGit = self.git_repo.get_workflows()?;
        all.append(&mut allGit);
        Ok(all)
    }

    fn save_workflow(&mut self, workflow: Workflow) -> WorkflowResult<()> {
        // only save to local. making copies of Workflows
        self.dir_repo.save_workflow(workflow)
    }

    fn delete_workflow(&mut self, name: &str) -> WorkflowResult<()> {
        self.dir_repo.delete_workflow(name)
    }

    fn query_workflows(&self, query: &str) -> WorkflowResult<Vec<Workflow>> {
        let mut result = vec![];
        if let Ok(mut allLocal) = self.dir_repo.query_workflows(query) {
            result.append(&mut allLocal);
        }
        if let Ok(mut allGit) = self.git_repo.query_workflows(query) {
            result.append(&mut allGit);
        }
        Ok(result)
    }
}
