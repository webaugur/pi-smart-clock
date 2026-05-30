pub const APP_NAME: &str = "Smart Clock";
pub const VERSION: &str = "0.2.0";

// Clock speeds
pub const CLOCK_SPEED_HZ: u32 = 270_000_000;
pub const VREG_VOLTAGE: f32 = 1.20;

// SAME Codes for alerts (up to 9)
pub const SAME_CODES: [&str; 9] = [
    "18019", // Example: Marion County, IN
    "18097", // Hamilton County
    "18057", // Hamilton County (example)
    "", "", "", "", "", ""
];

// Update intervals
pub const WEATHER_NORMAL_INTERVAL_HOURS: u64 = 3;
pub const WEATHER_ALERT_INTERVAL_MINUTES: u64 = 5;
pub const NON_WEATHER_INTERVAL_HOURS: u64 = 1;
pub const ALERT_VOICE_INTERVAL_MINUTES: u64 = 15;