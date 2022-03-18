use std::io::Error as IoError;
use std::process::Command;

pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur when controlling the master volume on a macOS system.
#[derive(Debug)]
pub enum Error {
    /// A standard I/O error.
    Io(IoError),
    /// Tried setting the master volume to an invalid amount.
    InvalidVolume(i64),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::InvalidVolume(vol) => write!(f, "could not set the master volume to {}", vol),
        }
    }
}

impl std::error::Error for Error {}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

/// Sets the master volume for this system.
///
/// Returns an error if volume is less than **0** or greater than **100**.
pub fn set(volume: i64) -> Result<()> {
    Command::new("osascript")
        .args(&["-e", &format!("set volume output volume {volume}")])
        .output()?;
    Ok(())
}
