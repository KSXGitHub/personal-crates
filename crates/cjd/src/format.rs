#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Format {
    Json,
    Yaml,
    Toml,
    Json5,
}

pub use Format::*;
