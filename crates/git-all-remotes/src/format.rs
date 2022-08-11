use clap::ValueEnum;
use derive_more::From;
use serde::Serialize;
use thiserror::Error;

/// Object notation format.
#[derive(Debug, Eq, PartialEq, Copy, Clone, ValueEnum)]
pub enum Format {
    Json,
    Yaml,
    Toml,
    Json5,
}

/// Error caused by [`Format::serialize`]
#[derive(Debug, From, Error)]
#[error("{}", _0)]
pub enum SerializationError {
    Json(serde_json::Error),
    Yaml(serde_yaml::Error),
    Toml(toml::ser::Error),
    Json5(json5::Error),
}

mod serialize {
    pub use json5::to_string as json5;
    pub use serde_json::to_string_pretty as json;
    pub use serde_yaml::to_string as yaml;
    pub use toml::to_string_pretty as toml;
}

impl Format {
    pub fn serialize(self, data: &impl Serialize) -> Result<String, SerializationError> {
        Ok(match self {
            Format::Json => serialize::json(data)?,
            Format::Yaml => serialize::yaml(data)?,
            Format::Toml => serialize::toml(data)?,
            Format::Json5 => serialize::json5(data)?,
        })
    }
}
