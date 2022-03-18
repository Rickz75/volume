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
    /// Could not parse the result from stdout.
    Parse(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::InvalidVolume(vol) => write!(f, "could not set the master volume to {}", vol),
            Error::Parse(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

/// Retrieves the master volume on this system.
pub fn get() -> Result<i64> {
    let output = Command::new("osascript")
        .args(&["-e", "output volume of (get volume settings)"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .trim()
        .parse::<i64>()
        .map_err(|e| Error::Parse(e.to_string()))
}

/// Sets the master volume for this system.
///
/// Returns an error if volume is less than **0** or greater than **100**.
pub fn set(volume: i64) -> Result<()> {
    if volume < 0 || volume > 100 {
        return Err(Error::InvalidVolume(volume));
    }
    Command::new("osascript")
        .args(&["-e", &format!("set volume output volume {volume}")])
        .output()?;
    Ok(())
}

/// Mutes the master volume on this system.
pub fn mute() -> Result<()> {
    Command::new("osascript")
        .args(&["-e", "set volume output muted true"])
        .output()?;
    Ok(())
}

/// Unmutes the master volume on this system.
pub fn unmute() -> Result<()> {
    Command::new("osascript")
        .args(&["-e", "set volume output muted false"])
        .output()?;
    Ok(())
}

/// Returns whether or not the master volume is currently muted.
pub fn is_muted() -> Result<bool> {
    let output = Command::new("osascript")
        .args(&["-e", "output muted of (get volume settings)"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    match stdout.trim() {
        "false" => Ok(false),
        "true" => Ok(true),
        out => Err(Error::Parse(format!(
            "expected true or false from output, but found '{}'",
            out
        ))),
    }
}
