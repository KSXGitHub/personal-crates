mod args;
mod display_result;
mod error;

use args::Args;
use clap::Parser;
use display_result::DisplayResult;
use error::Error;
use git2::Repository;
use os_display::Quotable;
use os_str_bytes::OsStrBytes;
use pipe_trait::Pipe;
use std::{env::current_dir, ffi::OsStr, io::stdout, path::PathBuf};

fn get_repo(repo: Option<PathBuf>) -> Result<Repository, Error> {
    if let Some(repo) = repo {
        Repository::open(repo)?.pipe(Ok)
    } else {
        current_dir()?.pipe(Repository::discover)?.pipe(Ok)
    }
}

fn app() -> Result<(), Error> {
    let Args { color, repo } = Parser::parse();
    let repo = get_repo(repo)?;
    let remotes = repo.remotes()?;
    let name_style = color.style(&stdout(), |style| style.bold());

    for name in remotes.iter().flatten() {
        let remote = repo.find_remote(name)?;
        let url = remote.url_bytes().pipe(OsStr::assert_from_raw_bytes);
        println!("{}: {}", name_style.paint(name), url.maybe_quote());
    }

    Ok(())
}

fn main() -> DisplayResult<(), Error> {
    DisplayResult(app())
}
