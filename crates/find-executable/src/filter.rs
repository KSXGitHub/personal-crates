use clap::ValueEnum;

/// Value to display.
#[derive(Debug, Clone, ValueEnum)]
pub enum Filter {
    /// Show only the original candidate.
    Word,
    /// Show only the path of the found item.
    Path,
    /// Show both as a JSON object.
    Both,
}
