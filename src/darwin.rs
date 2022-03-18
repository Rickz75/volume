use std::io::Error as IoError;

pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur when controlling the master volume on a macOS system.
#[derive(Debug)]
pub enum Error {
    /// A standard I/O error.
    Io(IoError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
