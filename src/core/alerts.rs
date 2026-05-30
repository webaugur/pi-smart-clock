use crate::drivers::platform::Platform;

pub struct AlertManager {
    pub same_codes: [String; 9],
    pub radar_active: bool,
    pub amber_silver_active: bool,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            same_codes: config::SAME_CODES.iter().map(|s| s.to_string()).collect::<Vec<_>>().try_into().unwrap(),
            radar_active: false,
            amber_silver_active: false,
        }
    }

    pub async fn check_nws_alerts<P: Platform>(&mut self, platform: &mut P) {
        // Query NWS API via ESP8266 every 5-15 min during alerts
    }
}