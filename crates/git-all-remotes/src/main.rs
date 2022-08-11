mod args;
mod display_result;
mod error;

use args::Args;
use clap::Parser;
use display_result::DisplayResult;
use error::Error;
use git2::Repository;
use nu_ansi_term::Style;
use os_display::Quotable;
use os_str_bytes::OsStrBytes;
use pipe_trait::Pipe;
use std::ffi::OsStr;

fn app() -> Result<(), Error> {
    let Args { repo } = Parser::parse();
    let repo = Repository::open(repo)?;
    let remotes = repo.remotes()?;

    let name_style = Style::new().bold();

    for name in remotes.iter().flatten() {
        let remote = repo.find_remote(name)?;
        let url = remote.url_bytes().pipe(OsStr::from_raw_bytes)?;
        println!("{}: {}", name_style.paint(name), url.maybe_quote());
    }

    Ok(())
}

fn main() -> DisplayResult<(), Error> {
    DisplayResult(app())
}
