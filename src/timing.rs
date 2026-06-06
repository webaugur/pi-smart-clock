#[cfg(feature = "linux-full")]
pub use std::time::{Duration, Instant};

#[cfg(not(feature = "linux-full"))]
mod embedded {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Duration(u64);

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Instant(u64);

    static mut TICK_MS: u64 = 0;

    pub fn advance_ms(ms: u64) {
        unsafe {
            TICK_MS = TICK_MS.saturating_add(ms);
        }
    }

    impl Duration {
        pub const fn from_secs(secs: u64) -> Self {
            Self(secs.saturating_mul(1000))
        }

        pub const fn from_millis(ms: u64) -> Self {
            Self(ms)
        }

        pub const fn as_millis(&self) -> u64 {
            self.0
        }
    }

    impl PartialOrd for Duration {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Duration {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl Instant {
        pub fn now() -> Self {
            Self(unsafe { TICK_MS })
        }

        pub fn duration_since(&self, earlier: Self) -> Duration {
            Duration(self.0.saturating_sub(earlier.0))
        }
    }

    impl core::ops::Sub<Duration> for Instant {
        type Output = Self;

        fn sub(self, rhs: Duration) -> Self::Output {
            Self(self.0.saturating_sub(rhs.0))
        }
    }
}

#[cfg(not(feature = "linux-full"))]
pub use embedded::{advance_ms, Duration, Instant};