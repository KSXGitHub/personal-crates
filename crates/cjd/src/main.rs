mod args;
mod format;

use args::Args;
use clap::Parser;
use format::*;
use serde_json::Value;
use std::{
    fmt::Display,
    io::{stdin, stdout, Read, Write},
    process::ExitCode,
};

fn display(x: impl Display) -> String {
    format!("{}", x)
}

fn app() -> Result<(), String> {
    let Args {
        input_format,
        output_format,
    } = Parser::parse();

    let stdin = stdin();

    macro_rules! from_reader {
        ($module:ident) => {
            $module::from_reader(stdin).map_err(display)
        };
    }

    macro_rules! from_str {
        ($module:ident) => {{
            let mut buf = String::new();
            let mut stdin = stdin;
            stdin.read_to_string(&mut buf).map_err(display)?;
            $module::from_str(&buf).map_err(display)
        }};
    }

    let value: Value = match input_format {
        Json => from_reader!(serde_json),
        Yaml => from_reader!(serde_yaml),
        Toml => from_str!(toml),
        Json5 => from_str!(json5),
    }?;

    let stdout = stdout();

    macro_rules! to_writer {
        ($module:ident) => {
            $module::to_writer(stdout, &value).map_err(display)
        };
    }

    macro_rules! to_string {
        ($module:ident) => {{
            let mut stdout = stdout;
            let text = $module::to_string(&value).map_err(display)?;
            write!(stdout, "{}", text).map_err(display)
        }};
    }

    match output_format {
        Json => to_writer!(serde_json),
        Yaml => to_writer!(serde_yaml),
        Toml => to_string!(toml),
        Json5 => to_string!(json5),
    }
}

fn main() -> ExitCode {
    match app() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
