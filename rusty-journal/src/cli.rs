use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
        task: String,
    },
    /// Remove an entry from the journal file by position.
    Done {
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

#[derive(Debug, Parser)]
#[command(name = "Rusty Journal", about = "A command line to-do app written in Rust")]
pub struct CommandLineArgs {
    #[command(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[arg(short, long, value_name = "FILE")]
    pub journal_file: Option<PathBuf>,
}