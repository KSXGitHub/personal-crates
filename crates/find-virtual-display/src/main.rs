mod sequence;
mod utils;

use clap::Parser;
use pipe_trait::*;
use sequence::Sequence;
use std::{
    iter::{IntoIterator, Iterator},
    path::PathBuf,
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

    let display = sequences
        .into_iter()
        .flatten()
        .map(|x| x.to_string())
        .find(|x| !format!("/tmp/.X{}-lock", x).pipe(PathBuf::from).exists());

    if let Some(display) = display {
        println!("{}", display);
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
