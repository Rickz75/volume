#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "macos", path = "darwin.rs")]
mod os;

pub use os::*;
