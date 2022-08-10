use command_extra::CommandExtra;
use derive_more::From;
use pipe_trait::Pipe;
use std::{
    ffi::OsStr,
    io,
    process::{Command, Output},
};
use thiserror::Error;

#[derive(Debug)]
pub enum CheckValue {
    Active { stdout: Vec<u8> },
    Inactive { stderr: Vec<u8> },
}

#[derive(Debug, From, Error)]
pub enum CheckError {
    #[error("{}", _0)]
    Io(io::Error),
    #[error("unknown error: {}", _0)]
    Unknown(String),
}

pub fn check_virtual_display(display: impl AsRef<OsStr>) -> Result<CheckValue, CheckError> {
    let Output {
        status,
        stdout,
        stderr,
    } = Command::new("xset")
        .with_arg("q")
        .with_arg("-display")
        .with_arg(display)
        .output()?;

    if status.success() {
        return Ok(CheckValue::Active { stdout });
    }

    let error_message = String::from_utf8_lossy(&stderr);

    if error_message.contains("unable to open display") {
        return Ok(CheckValue::Inactive { stderr });
    }

    error_message
        .to_string()
        .pipe(CheckError::Unknown)
        .pipe(Err)
}
