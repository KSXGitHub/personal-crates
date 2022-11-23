mod execute_command;
mod handle_output;

use clap::Parser;
use handle_output::HandleOutput;
use portpicker::{is_free, pick_unused_port, Port};
use sequence::Sequence;
use std::{ffi::OsString, process::ExitCode};

/// Find an unoccupied X11 display port.
#[derive(Parser)]
struct CliArgs {
    /// Execute a command with $FOUND_UNUSED_PORT.
    #[clap(long = "exec", short = 'x', raw = true)]
    command: Vec<OsString>,
    /// Sequences of port ranges, e.g. 5000-9999.
    sequences: Vec<Sequence>,
}

fn main() -> ExitCode {
    let CliArgs { command, sequences } = Parser::parse();

    let port = match find_unused_port(sequences) {
        Some(port) => port,
        None => {
            eprintln!("error: Failed to find an unused port");
            return ExitCode::FAILURE;
        }
    };

    HandleOutput::builder()
        .port(port)
        .command(command)
        .build()
        .run()
        .unwrap_or_else(|error| {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        })
}

fn find_unused_port(potential_ports: Vec<Sequence>) -> Option<u16> {
    if potential_ports.is_empty() {
        return pick_unused_port().map(From::from);
    }

    potential_ports
        .into_iter()
        .flatten()
        .flat_map(Port::try_from)
        .find(|port| is_free(*port))
}
