use crate::ui::components::{Action, Component};
use crate::ui::AppState;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::prelude::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use crate::prelude::repository::WorkflowRepository;

pub struct CommandListComponent;

impl CommandListComponent {
    pub fn new() -> Box<Self> {
        Box::new(CommandListComponent {})
    }
}

impl Component for CommandListComponent {
    fn render(&self, _state: &AppState, frame: &mut Frame, _rect: Rect) {
        let area = frame.size();

        // get list of all items from app state
        let _commands = _state.get_workflows();

        frame.render_widget(
            Paragraph::new("Eine andere Message! (press 'q' to quit)")
                .black()
                .on_white(),
            area,
        );
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        if key.kind == KeyEventKind::Press
            && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
        {
            println!("Quit Application");
            Action::Quit
        } else {
            Action::None
        }
    }
}
