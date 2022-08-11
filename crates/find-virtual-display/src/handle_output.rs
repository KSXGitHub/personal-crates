use super::execute_command::ExecuteCommand;
use derive_more::From;
use std::{ffi::OsString, io, process::ExitCode};
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct HandleOutput {
    display: (u32, String),
    command: Vec<OsString>,
}

#[derive(Debug, From, Error)]
pub enum HandleOutputError {
    #[error("{}", _0)]
    Io(io::Error),
}

impl HandleOutput {
    pub fn run(self) -> Result<ExitCode, HandleOutputError> {
        let HandleOutput { display, command } = self;
        let mut cmd_iter = command.into_iter();
        if let Some(program) = cmd_iter.next() {
            ExecuteCommand::builder()
                .display(display)
                .program(program)
                .arguments(cmd_iter)
                .build()
                .run()
                .map_err(From::from)
        } else {
            let (_, display_string): (u32, String) = display;
            println!("{display_string}");
            Ok(ExitCode::SUCCESS)
        }
    }
}
