use super::format::*;
use std::hint::unreachable_unchecked;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(
        name = "input format",
        possible_values = &["json", "yaml", "toml", "json5"],
        parse(from_str = parse_format),
)]
    pub input_format: Format,
    #[structopt(
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
