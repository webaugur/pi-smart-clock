//! Wall-clock helpers. Linux uses chrono Local; Pico uses UTC via chrono or
//! a lightweight counter until RTC/DS3231 is wired.

#[cfg(feature = "linux-full")]
pub use chrono::{DateTime, Local};

#[cfg(not(feature = "linux-full"))]
#[derive(Clone, Copy, Debug)]
pub struct WallTime {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

#[cfg(not(feature = "linux-full"))]
impl WallTime {
    pub const fn new(hour: u32, minute: u32, second: u32) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }
}