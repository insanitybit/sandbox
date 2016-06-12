#![deny(warnings)]

use nix::Error as nixError;

use std::error;
use std::fmt;
use std::io::Error as ioError;


#[derive(Debug)]
pub enum SandboxError {
    IOError(ioError),
    NixError(nixError),
}

impl fmt::Display for SandboxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SandboxError::IOError(ref err) => write!(f, "IO error: {}", err),
            SandboxError::NixError(ref err) => write!(f, "Nix error: {}", err),
        }
    }
}

impl error::Error for SandboxError {
    fn description(&self) -> &str {
        match *self {
            SandboxError::IOError(ref err) => err.description(),
            SandboxError::NixError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SandboxError::IOError(ref err) => Some(err),
            SandboxError::NixError(ref err) => Some(err),
        }
    }
}

impl From<nixError> for SandboxError {
    fn from(err: nixError) -> SandboxError {
        SandboxError::NixError(err)
    }
}

impl From<ioError> for SandboxError {
    fn from(err: ioError) -> SandboxError {
        SandboxError::IOError(err)
    }
}
