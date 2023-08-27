use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[clap(about = "Parse output of `git status --porcelain` via stdin")]
pub enum CliArgs {
    #[clap(about = "Select a column to display")]
    Select {
        #[clap(help = "Column to display")]
        column: Column,
    },
    #[clap(about = "Show all fields as Rust objects")]
    All,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Column {
    #[clap(help = "Staged status")]
    Staged,
    #[clap(help = "Unstaged status")]
    Unstaged,
    #[clap(help = "Path to file or directory")]
    Path,
}
