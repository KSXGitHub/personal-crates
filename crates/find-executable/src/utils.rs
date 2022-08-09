use libc::ENOENT;
use std::{ffi::OsString, path::PathBuf, process::ExitCode};
use which::{which, Error::CannotFindBinaryPath};

/// Find an existing executable.
pub fn find(candidates: Vec<OsString>) -> Result<(OsString, PathBuf), which::Error> {
    for word in candidates {
        match which(&word) {
            Ok(path) => return Ok((word, path)),
            Err(CannotFindBinaryPath) => continue,
            Err(error) => return Err(error),
        }
    }
    Err(CannotFindBinaryPath)
}

/// Convert [`which::Error`] to an [exit code](ExitCode).
pub fn which_error_to_exit_code(error: which::Error) -> ExitCode {
    if error == CannotFindBinaryPath {
        ExitCode::from(ENOENT as u8)
    } else {
        ExitCode::FAILURE
    }
}
