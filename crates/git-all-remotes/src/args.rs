use clap::Parser;
use std::path::PathBuf;

/// Show all remotes in a repository.
#[derive(Debug, Parser)]
pub struct Args {
    /// Root of the repository.
    #[clap(default_value = ".")]
    pub repo: PathBuf,
}
