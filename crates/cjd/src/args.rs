use super::format::*;
use clap::Parser;
use std::hint::unreachable_unchecked;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(
        name = "input format",
        possible_values = &["json", "yaml", "toml", "json5"],
        parse(from_str = parse_format),
    )]
    pub input_format: Format,
    #[clap(
        name = "output format",
        possible_values = &["json", "yaml", "toml", "json5"],
        parse(from_str = parse_format),
    )]
    pub output_format: Format,
}

fn parse_format(argument: &str) -> Format {
    match argument {
        "json" => Json,
        "yaml" => Yaml,
        "toml" => Toml,
        "json5" => Json5,
        _ => unsafe { unreachable_unchecked() },
    }
}
