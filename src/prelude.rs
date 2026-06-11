//! Re-exports and common items for the desktop build (std always available).
//! Pico/embedded support has been removed.

pub use std::string::{String, ToString};
pub use std::vec::Vec;

pub use crate::timing::{Duration, Instant};