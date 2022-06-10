mod display_result;
mod error;

use ansi_term::Style;
use display_result::DisplayResult;
use error::Error;
use git2::Repository;
use pipe_trait::*;
use std::env::current_dir;

fn app() -> Result<(), Error> {
    let repo = current_dir()
        .map_err(Error::Os)?
        .pipe(Repository::open)
        .map_err(Error::Git)?;

    let remotes = repo.remotes().map_err(Error::Git)?;

    let name_style = Style::new().bold();

    for name in remotes.iter().flatten() {
        let remote = repo.find_remote(name).map_err(Error::Git)?;
        let url = remote.url().expect("get url from remote");
        println!("{}: {}", name_style.paint(name), url);
    }

    Ok(())
}

fn main() -> DisplayResult<(), Error> {
    DisplayResult(app())
}
