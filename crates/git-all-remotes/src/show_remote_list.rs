use crate::{error::Error, format::Format};
use git2::Repository;
use nu_ansi_term::Style;
use os_display::Quotable;
use os_str_bytes::OsStrBytes;
use pipe_trait::Pipe;
use std::{collections::BTreeMap, ffi::OsStr};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct ShowRemoteList {
    repo: Repository,
    format: Option<Format>,
}

impl ShowRemoteList {
    pub fn run(self) -> Result<(), Error> {
        let ShowRemoteList { repo, format } = self;

        let names = repo.remotes()?;
        let names = names.iter().flatten();

        if let Some(format) = format {
            let mut remotes = BTreeMap::new();
            for name in names {
                let remote = repo.find_remote(name)?;
                let url = remote
                    .url_bytes()
                    .to_vec()
                    .pipe(OsStr::from_raw_bytes)?
                    .maybe_quote()
                    .to_string();
                remotes.insert(name, url);
            }
            let text = format.serialize(&remotes)?;
            println!("{text}");
        } else {
            let name_style = Style::new().bold();
            for name in names {
                let remote = repo.find_remote(name)?;
                let url = remote.url_bytes().pipe(OsStr::from_raw_bytes)?;
                println!("{}: {}", name_style.paint(name), url.maybe_quote());
            }
        }

        Ok(())
    }
}
