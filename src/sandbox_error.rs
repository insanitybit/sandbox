#![deny(warnings)]

use nix::Error as nixError;

use std::error;
use std::fmt;
use std::io::Error as ioError;

#[derive(Debug)]
pub enum SandboxError {
    IOError(ioError),
    NixError(nixError),
    CommunicationError(String),
    SandboxFailure(String),
}

impl fmt::Display for SandboxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SandboxError::IOError(ref err) => write!(f, "IO error: {}", err),
            SandboxError::NixError(ref err) => write!(f, "Nix error: {}", err),
            SandboxError::CommunicationError(ref err) => {
                write!(f, "CommunicationError error: {}", err)
            }
            SandboxError::SandboxFailure(ref err) => write!(f, "CommunicationError error: {}", err),
        }
    }
}

impl error::Error for SandboxError {
    fn description(&self) -> &str {
        match *self {
            SandboxError::IOError(ref err) => err.description(),
            SandboxError::NixError(ref err) => err.description(),
            SandboxError::CommunicationError(ref err) => err,
            SandboxError::SandboxFailure(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SandboxError::IOError(ref err) => Some(err),
            SandboxError::NixError(ref err) => Some(err),
            SandboxError::CommunicationError(_) => None,
            SandboxError::SandboxFailure(_) => None,
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
