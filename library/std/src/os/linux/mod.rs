//! Linux-specific definitions.

#![stable(feature = "raw_ext", since = "1.1.0")]
#![doc(cfg(target_os = "linux"))]

pub mod fs;
pub mod raw;
#[cfg(target_os = "linux")]
pub mod net;
#[cfg(target_os = "linux")]
pub mod process;
