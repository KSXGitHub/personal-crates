use std::{
    fmt::Display,
    process::{ExitCode, Termination},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DisplayResult<Value, Error>(pub Result<Value, Error>);

impl<Error> Termination for DisplayResult<(), Error>
where
    Error: Display + Termination,
{
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("ERROR: {error}");
                error.report()
            }
        }
    }
}
