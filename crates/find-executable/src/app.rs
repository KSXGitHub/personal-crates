use crate::{filter::Filter, utils::*};
use clap::Parser;
use os_display::Quotable;
use serde_json::json;
use std::{ffi::OsString, path::Path, process::ExitCode};

/// Find an existing executable.
#[derive(Debug, Parser)]
pub struct App {
    /// Value to display.
    #[clap(long, short, value_enum, default_value_t = Filter::Path)]
    filter: Filter,
    /// List of commands to check.
    candidates: Vec<OsString>,
}

impl App {
    /// Run the finder.
    pub fn run(self) -> Result<(), which::Error> {
        let App { filter, candidates } = App::parse();
        let (word, path) = find(candidates)?;
        match filter {
            Filter::Word => println!("{}", word.maybe_quote()),
            Filter::Path => println!("{}", path.maybe_quote()),
            Filter::Both => println!("{}", json!({ "word": Path::new(&word), "path": path })),
        }
        Ok(())
    }

    /// The main program.
    pub fn main() -> ExitCode {
        let error = match App::parse().run() {
            Ok(()) => return ExitCode::SUCCESS,
            Err(error) => error,
        };
        eprintln!("{error}");
        which_error_to_exit_code(error)
    }
}
