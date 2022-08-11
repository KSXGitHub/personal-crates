use command_extra::CommandExtra;
use pipe_trait::Pipe;
use std::{
    ffi::OsString,
    io,
    process::{Command, ExitCode, Stdio},
};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct ExecuteCommand<Arguments> {
    display: (u32, String),
    program: OsString,
    arguments: Arguments,
}

impl<Arguments> ExecuteCommand<Arguments> {
    pub fn run(self) -> Result<ExitCode, io::Error>
    where
        Arguments: Iterator<Item = OsString>,
    {
        let ExecuteCommand {
            display: (display_number, display_string),
            program,
            arguments,
        } = self;

        Command::new(program)
            .with_stdin(Stdio::inherit())
            .with_stdout(Stdio::inherit())
            .with_stderr(Stdio::inherit())
            .with_env("DISPLAY_NUMBER", display_number.to_string())
            .with_env("DISPLAY_STRING", display_string)
            .with_args(arguments)
            .output()?
            .status
            .code()
            .unwrap_or(i32::max_value())
            .pipe(u8::try_from)
            .unwrap_or(u8::max_value())
            .pipe(ExitCode::from)
            .pipe(Ok)
    }
}
