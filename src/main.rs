use std::path::PathBuf;
// use clap::Parser;

// use crate::command::{Cli, Commands, HandleCommand};

use crossterm::{
    event::{self},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::layout::Rect;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};

use crate::ui::components::Action;
use crate::ui::components::Action::Quit;
use crate::ui::Focus::List;
use crate::ui::{AppState, Focus};

mod command;
mod prelude;
mod workflow;

mod ui;

fn main() -> Result<()> {
    // run and save results
    let result = run();
    // make sure to call shutdown
    shutdown_terminal()?;
    // and finally return the result
    result?;
    Ok(())
}

fn run() -> Result<()> {
    // create app state holding git_repo and local dir_repo
    let mut state = AppState::new(
        "https://github.com/warpdotdev/workflows.git",
        "main",
        PathBuf::from("tests/fixtures/github/warpdotdev"),
        PathBuf::from("tests/fixtures/workflows"),
    );

    init_terminal()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    loop {
        terminal.draw(|frame|
            match state.focus {
            Focus::List => state
                .command_list_component
                .render(&state, frame, Rect::default()),
        })?;

        let action = update(&mut state);

        if let Quit = action {
            break;
        }
    }
    Ok(())
}

fn update(app_state: &mut AppState) -> Action {
    if event::poll(std::time::Duration::from_millis(16)).unwrap() {
        match app_state.focus {
            List => {
                if let Ok(event) = event::read() {
                    return app_state.command_list_component.handle_events(Some(event));
                }
            }
        }
    }
    Action::None
}

fn shutdown_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn init_terminal() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(())
}
