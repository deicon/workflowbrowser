use crate::ui::components::{Action, Component};
use crate::ui::AppState;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Margin, Rect};
use ratatui::widgets::{Block, Borders, List, Paragraph};
use ratatui::Frame;
use ratatui::prelude::{Layout, Widget};
use crate::main;
use crate::prelude::repository::WorkflowRepository;

pub struct CommandListComponent;

impl CommandListComponent {
    pub fn new() -> Box<Self> {
        Box::new(CommandListComponent {})
    }
}


impl Component for CommandListComponent {
    fn render(&self, _state: &AppState, frame: &mut Frame, area: Rect) {
        let area = frame.size();

        // get list of all items from app state
        let _commands = _state.get_workflows();

        // create simple layout, having a single line at the end of the
        // screen to input.
        let layout
            = Layout::default()
            .constraints([
                Constraint::Min(1),
                Constraint::Max(3),
            ].into_iter()).split(frame.size());

        let main_screen = layout[0];
        let input_line = layout[1];

        let area = area.inner(&Margin {
            vertical: 1,
            horizontal: 2
        });

        frame.render_widget(
            Paragraph::new("Top")
                .block(Block::new().borders(Borders::ALL)),
            main_screen);

        frame.render_widget(
            Paragraph::new("Bottom")
                .block(Block::new().borders(Borders::ALL)),
            input_line);

        // frame.render_widget(
        //     List::new(_commands.unwrap()),
        //     area,
        // );
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
