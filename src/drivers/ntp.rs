use crate::drivers::platform::Platform;

pub struct NtpClient;

impl NtpClient {
    pub async fn sync<P: Platform>(platform: &mut P) -> Result<(), String> {
        // Uses ESP8266 to fetch time from pool.ntp.org
        if let Some(time_str) = platform.esp8266_get_ntp("pool.ntp.org").await {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&time_str) {
                // In real implementation, set DS3231 time here
                println!("🌍 NTP Sync successful: {}", dt);
                return Ok(());
            }
        }
        Err("NTP sync failed".to_string())
    }
}