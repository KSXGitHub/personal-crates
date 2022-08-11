mod args;
mod display_result;
mod error;
mod format;
mod show_remote_list;

use args::Args;
use clap::Parser;
use display_result::DisplayResult;
use error::Error;
use git2::Repository;
use show_remote_list::ShowRemoteList;

fn app() -> Result<(), Error> {
    let Args { repo, format } = Parser::parse();
    let repo = Repository::open(repo)?;

    ShowRemoteList::builder()
        .repo(repo)
        .format(format)
        .build()
        .run()
}

fn main() -> DisplayResult<(), Error> {
    DisplayResult(app())
}
