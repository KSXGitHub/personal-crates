use crate::format::SerializationError;
use derive_more::From;
use std::{
    io,
    process::{ExitCode, Termination},
};
use thiserror::Error;

#[derive(Debug, From, Error)]
pub enum Error {
    #[error("{}", _0)]
    Os(io::Error),
    #[error("{}", _0)]
    Git(git2::Error),
    #[error("{}", _0)]
    OsStrBytes(os_str_bytes::EncodingError),
    #[error("{}", _0)]
    Serialize(SerializationError),
}

impl Error {
    fn code(&self) -> i32 {
        match self {
            Error::Os(error) => error.raw_os_error().unwrap_or(1),
            Error::Git(error) => error.raw_code(),
            Error::OsStrBytes(_) => 1,
            Error::Serialize(_) => 1,
        }
    }
}

impl Termination for Error {
    fn report(self) -> ExitCode {
        ExitCode::from(self.code() as u8)
    }
}
