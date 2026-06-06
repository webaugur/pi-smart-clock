use crate::drivers::platform::Platform;
use crate::prelude::*;

pub struct NtpClient;

impl NtpClient {
    pub async fn sync<P: Platform>(platform: &mut P) -> Result<(), String> {
        if let Some(time_str) = platform.esp8266_get_ntp("pool.ntp.org").await {
            #[cfg(feature = "linux-full")]
            if chrono::DateTime::parse_from_rfc3339(&time_str).is_ok() {
                println!("🌍 NTP Sync successful: {time_str}");
                return Ok(());
            }
            #[cfg(not(feature = "linux-full"))]
            if !time_str.is_empty() {
                let _ = time_str;
                return Ok(());
            }
        }
        Err(String::from("NTP sync failed"))
    }
}