use crate::drivers::platform::Platform;

pub struct Logger {
    pub enabled: bool,
}

impl Logger {
    pub fn new() -> Self {
        Self { enabled: false } // Off by default
    }

    pub async fn log<P: Platform>(&self, platform: &mut P, level: &str, message: &str) {
        if !self.enabled { return; }
        let timestamp = platform.get_current_time();
        #[cfg(feature = "full")]
        println!("[{}] {}: {}", timestamp.format("%H:%M:%S"), level, message);
        #[cfg(not(feature = "full"))]
        let _ = (timestamp, level, message);
    }
}