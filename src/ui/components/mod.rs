pub mod command_list;

use crate::prelude::WorkflowResult;
use crate::ui::AppState;
use crossterm::event::{Event, KeyEvent, MouseEvent};
use ratatui::prelude::Rect;
use ratatui::Frame;

pub enum Action {
    None,
    Quit,
}

pub trait Component {
    fn init(&mut self, _state: AppState) -> WorkflowResult<()> {
        Ok(())
    }
    fn handle_events(&mut self, event: Option<Event>) -> Action {
        match event {
            Some(Event::Key(key_event)) => self.handle_key_events(key_event),
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event),
            _ => Action::None,
        }
    }
    fn handle_key_events(&mut self, _key: KeyEvent) -> Action {
        Action::None
    }
    fn handle_mouse_events(&mut self, _mouse: MouseEvent) -> Action {
        Action::None
    }
    fn update(&mut self, _state: &mut AppState, _action: Action) -> Action {
        Action::None
    }
    fn render(&self, state: &AppState, f: &mut Frame, rect: Rect);
}
