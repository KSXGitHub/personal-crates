use clap::{Parser, ValueEnum};
use is_terminal::IsTerminal;
use nu_ansi_term::Style;
use std::path::PathBuf;

/// Show all remotes in a repository.
#[derive(Debug, Parser)]
pub struct Args {
    /// When to use terminal color.
    #[clap(long, value_enum, default_value_t = When::Auto, name = "WHEN")]
    pub color: When,
    /// Root of the repository.
    #[clap(default_value = ".")]
    pub repo: PathBuf,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum When {
    Always,
    Auto,
    Never,
}

impl When {
    pub fn style(self, stream: &impl IsTerminal, style: impl FnOnce(Style) -> Style) -> Style {
        let blank = Style::new();
        match self {
            When::Never => return blank,
            When::Always => return style(blank),
            When::Auto => {}
        }
        if stream.is_terminal() {
            style(blank)
        } else {
            blank
        }
    }
}
