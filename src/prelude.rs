//! Imports for embedded (`pico-dvi`) builds.

#[cfg(not(feature = "linux-full"))]
pub use alloc::string::{String, ToString};
#[cfg(not(feature = "linux-full"))]
pub use alloc::vec::Vec;

#[cfg(not(feature = "linux-full"))]
pub use embassy_time::{Duration, Instant};