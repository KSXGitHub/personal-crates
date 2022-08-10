use super::format::*;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Format of the input.
    #[clap(value_enum)]
    pub input_format: Format,
    /// Format of the output.
    #[clap(value_enum)]
    pub output_format: Format,
}
