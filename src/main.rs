use std::path::PathBuf;
// use clap::Parser;
use di::{Ref, ServiceCollection, singleton_factory};
// use crate::command::{Cli, Commands, HandleCommand};

use crate::prelude::directory::DirectoryRepository;
use crate::prelude::git::GitRepository;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{Frame, prelude::{CrosstermBackend, Stylize, Terminal}, widgets::Paragraph};
use std::io::{stdout, Result};
use crate::prelude::WorkflowResult;
use crate::ui::AppState;

mod prelude;
mod workflow;
mod command;

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
        PathBuf::from("tests/fixtures/workflows"));

    init_terminal()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    loop {
        terminal.draw(|frame| {
            ui(&mut state, frame);
        })?;
        update(&mut state)?;

        if state.should_quit {
            break;
        }
    }
    Ok(())
}

fn update(app_state: &mut AppState) -> Result<()>{
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
            {
                app_state.should_quit = true;
            }
        }
    }
    Ok(())
}

fn ui(_: &mut AppState, frame: &mut Frame) {
    let area = frame.size();
    frame.render_widget(
        Paragraph::new("Hello Ratatui! (press 'q' to quit)")
            .black()
            .on_white(),
            area,
    );
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
