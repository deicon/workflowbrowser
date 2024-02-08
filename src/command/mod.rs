mod search_command;

use clap::{Args, Parser, Subcommand};
use di::ServiceProvider;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Search workflows by query
    Search(SearchCommand)
}

#[derive(Debug, Args)]
pub struct SearchCommand {
    #[arg(short, long)]
    pub query: String
}

pub trait HandleCommand {
    fn handle(&self, repo: &ServiceProvider);
}