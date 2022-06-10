use std::{
    fmt::{self, Display, Formatter},
    io,
    process::{ExitCode, Termination},
};

pub enum Error {
    Os(io::Error),
    Git(git2::Error),
}

impl Error {
    fn code(&self) -> i32 {
        match self {
            Error::Os(error) => error.raw_os_error().unwrap_or(1),
            Error::Git(error) => error.raw_code(),
        }
    }
}

impl Termination for Error {
    fn report(self) -> ExitCode {
        ExitCode::from(self.code() as u8)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Os(error) => write!(f, "{}", error),
            Error::Git(error) => write!(f, "{}", error),
        }
    }
}
