mod check_virtual_display;
mod sequence;
mod utils;

use check_virtual_display::*;
use clap::Parser;
use sequence::Sequence;
use std::{
    iter::{IntoIterator, Iterator},
    process::ExitCode,
};

#[derive(Parser)]
struct CliArgs {
    /// Sequences of display id ranges, e.g. 0-3
    #[clap(default_value = "255-0")]
    sequences: Vec<Sequence>,
}

fn main() -> ExitCode {
    let CliArgs { sequences } = CliArgs::parse();

    let display_iter = sequences.into_iter().flatten().map(|x| format!(":{x}"));

    for display in display_iter {
        match check_virtual_display(&display) {
            Ok(CheckValue::Active { stdout: _ }) => continue,
            Ok(CheckValue::Inactive { stderr: _ }) => {
                println!("{display}");
                return ExitCode::SUCCESS;
            }
            Err(error) => {
                eprintln!("warn: (display: {display}) {error}");
                continue;
            }
        }
    }

    ExitCode::FAILURE
}
