mod check_virtual_display;
mod execute_command;
mod handle_output;
mod sequence;
mod utils;

use check_virtual_display::*;
use clap::Parser;
use handle_output::HandleOutput;
use sequence::Sequence;
use std::{
    ffi::OsString,
    iter::{IntoIterator, Iterator},
    process::ExitCode,
};

/// Find an unoccupied X11 display port.
#[derive(Parser)]
struct CliArgs {
    /// Execute a command.
    #[clap(long = "exec", short = 'x', multiple_values = true, raw = true)]
    command: Vec<OsString>,
    /// Sequences of display id ranges, e.g. 0-3.
    #[clap(default_value = "255-0")]
    sequences: Vec<Sequence>,
}

fn main() -> ExitCode {
    let CliArgs { command, sequences } = CliArgs::parse();

    let display_iter = sequences.into_iter().flatten().map(|x| format!(":{x}"));

    for display in display_iter {
        match check_virtual_display(&display) {
            Ok(CheckValue::Active { stdout: _ }) => continue,
            Ok(CheckValue::Inactive { stderr: _ }) => {
                return HandleOutput::builder()
                    .display(display)
                    .command(command)
                    .build()
                    .run()
                    .unwrap_or_else(|error| {
                        eprintln!("error: {error}");
                        ExitCode::FAILURE
                    });
            }
            Err(error) => {
                eprintln!("warn: (display: {display}) {error}");
                continue;
            }
        }
    }

    ExitCode::FAILURE
}
