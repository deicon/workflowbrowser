use std::path::PathBuf;
use crossterm::event::{Event, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::Rect;

use crate::prelude::directory::DirectoryRepository;
use crate::prelude::git::GitRepository;
use crate::prelude::WorkflowResult;
use crate::ui::components::command_list::CommandListComponent;
use crate::ui::components::{Action, Component};

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

impl Component for AppState {

    fn handle_events(&mut self, event: Option<Event>) -> Action {
        todo!()
    }

    fn handle_key_events(&mut self, _key: KeyEvent) -> Action {
        todo!()
    }

    fn handle_mouse_events(&mut self, _mouse: MouseEvent) -> Action {
        todo!()
    }

    fn update(&mut self, _state: &mut AppState, _action: Action) -> Action {
        todo!()
    }

    fn render(&self, state: &AppState, f: &mut Frame, rect: Rect) {
        todo!()
    }
}
