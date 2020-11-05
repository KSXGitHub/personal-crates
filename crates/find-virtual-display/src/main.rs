mod sequence;
mod utils;

use pipe_trait::*;
use sequence::Sequence;
use std::{
    iter::{IntoIterator, Iterator},
    path::PathBuf,
    process::exit,
};
use structopt::StructOpt;
use structopt_utilities::StructOptUtils;

#[derive(StructOpt)]
struct CliArgs {
    /// Sequences of display id ranges, e.g. 0-3
    #[structopt(default_value = "255-0")]
    sequences: Vec<Sequence>,
}

fn main() {
    let CliArgs { sequences } = CliArgs::strict_from_args();

    let display = sequences
        .into_iter()
        .flatten()
        .map(|x| x.to_string())
        .find(|x| !format!("/tmp/.X{}-lock", x).pipe(PathBuf::from).exists());

    if let Some(display) = display {
        println!("{}", display);
    } else {
        exit(1);
    }
}
