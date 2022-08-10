use clap::ValueEnum;

#[derive(Debug, Eq, PartialEq, Copy, Clone, ValueEnum)]
pub enum Format {
    Json,
    Yaml,
    Toml,
    Json5,
}

pub use Format::*;
