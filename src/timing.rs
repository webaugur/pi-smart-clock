#[cfg(feature = "linux-full")]
pub use std::time::{Duration, Instant};

#[cfg(not(feature = "linux-full"))]
pub use embassy_time::{Duration, Instant};