use crate::format::Format;
use clap::Parser;
use std::path::PathBuf;

/// Show all remotes in a repository.
#[derive(Debug, Parser)]
pub struct Args {
    /// Format the output as an object notation.
    #[clap(long, short, value_enum)]
    pub format: Option<Format>,
    /// Root of the repository.
    #[clap(default_value = ".")]
    pub repo: PathBuf,
}
