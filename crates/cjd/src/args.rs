use super::format::*;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(value_enum)]
    pub input_format: Format,
    #[clap(value_enum)]
    pub output_format: Format,
}
