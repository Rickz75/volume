use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

/// An error that can occur when controlling the master volume on a linux system.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Could not find the default card to use as the mixer.
    NoDefault,
    /// Could not find the master element.
    NoMaster,
    /// Could not find the first channel on the master element.
    NoChannel,
    /// The master volume cannot be muted or unmuted on this system.
    NoPlayBackSwitch,
    /// Tried setting the master volume with an invalid integer.
    InvalidVolume(i64),
    /// An internal error. User should file a bug report.
    Internal,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match *self {
            NoDefault => f.write_str("could not find the default audio card with alsa"),
            NoMaster => f.write_str("could not get a handle on the master volume"),
            NoChannel => f.write_str("no default channel found on the mixer"),
            NoPlayBackSwitch => f.write_str("cannot mute/unmute the master volume on the system"),
            InvalidVolume(vol) => write!(f, "could not set the master volume to {}", vol),
            Internal => f.write_str("an internal error occurred, please file a bug report"),
        }
    }
}

impl error::Error for Error {}

impl Error {
    fn from(code: i32) -> Option<Error> {
        match code {
            0 => None,
            1 => Some(Error::NoDefault),
            2 => Some(Error::NoMaster),
            3 => Some(Error::NoChannel),
            4 => Some(Error::NoPlayBackSwitch),
            _ => Some(Error::Internal),
        }
    }
}

/// Get the master volume on this system.
pub fn get() -> Result<i64> {
    let mut out = 0;
    let ptr = &mut out as *mut i64;
    unsafe {
        if let Some(err) = Error::from(alsa::get_volume(ptr)) {
            Err(err)
        } else {
            Ok(out)
        }
    }
}

/// Set the master volume for this system.
///
/// Returns an error if volume is less than **0** or greater than **100**.
pub fn set(volume: i64) -> Result<()> {
    if volume < 0 || volume > 100 {
        return Err(Error::InvalidVolume(volume));
    }
    unsafe {
        if let Some(err) = Error::from(alsa::set_volume(volume)) {
            Err(err)
        } else {
            Ok(())
        }
    }
}

/// Mute the master volume on this system.
pub fn mute() -> Result<()> {
    unsafe {
        if let Some(err) = Error::from(alsa::mute()) {
            Err(err)
        } else {
            Ok(())
        }
    }
}

/// Unmute the master volume on this system.
pub fn unmute() -> Result<()> {
    unsafe {
        if let Some(err) = Error::from(alsa::unmute()) {
            Err(err)
        } else {
            Ok(())
        }
    }
}

/// Check whether or not the master volume is muted.
pub fn is_muted() -> Result<bool> {
    let mut out = 2;
    let ptr = &mut out as *mut i32;
    unsafe {
        if let Some(err) = Error::from(alsa::is_muted(ptr)) {
            Err(err)
        } else {
            match out {
                0 => Ok(true),
                1 => Ok(false),
                _ => Err(Error::Internal),
            }
        }
    }
}

mod alsa {
    #[link(name = "alsa")]
    extern "C" {
        pub fn get_volume(out: *mut i64) -> i32;
        pub fn set_volume(vol: i64) -> i32;
        pub fn mute() -> i32;
        pub fn unmute() -> i32;
        pub fn is_muted(out: *mut i32) -> i32;
    }
}
