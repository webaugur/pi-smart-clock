pub const APP_NAME: &str = "Smart Clock";
pub const VERSION: &str = "0.1.0";

// Clock speeds (for Pico DVI)
pub const CLOCK_SPEED_HZ: u32 = 270_000_000;  // 270 MHz (recommended with copper heatsink)
pub const VREG_VOLTAGE: f32 = 1.20;
pub const FLASH_DIVIDER: u8 = 2;

// MQTT (LAN)
pub const MQTT_BROKER: &str = "192.168.1.100";
pub const MQTT_PORT: u16 = 1883;
pub const MQTT_USER: Option<&str> = Some("homeassistant");
pub const MQTT_PASS: Option<&str> = Some("your_password_here");

// SAME codes for alerts (up to 9)
pub const SAME_CODES: [&str; 9] = [
    "18003", // Example: Marion County, IN
    "18097", // Hamilton County, IN
    "", "", "", "", "", "", ""
];

// Night mode
pub const NIGHT_MODE_START_HOUR: u32 = 22; // 10 PM
pub const NIGHT_MODE_END_HOUR: u32 = 6;   // 6 AM

// OTA
pub const OTA_ENABLED_BY_DEFAULT: bool = false;
