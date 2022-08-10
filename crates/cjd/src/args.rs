use super::format::*;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(name = "input format", value_enum)]
    pub input_format: Format,
    #[clap(name = "output format", value_enum)]
    pub output_format: Format,
}
