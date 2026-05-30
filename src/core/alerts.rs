use crate::drivers::platform::Platform;
use std::time::Instant;

pub struct AlertManager {
    pub same_codes: [String; 9],
    pub radar_active: bool,
    pub amber_silver_active: bool,
    pub force_radar: bool,
    last_check: Option<Instant>,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            same_codes: Default::default(),
            radar_active: false,
            amber_silver_active: false,
            force_radar: false,
            last_check: None,
        }
    }

    pub async fn check_nws_alerts<P: Platform>(&mut self, platform: &mut P) {
        // In real implementation this would call ESP8266 to query NWS CAP API
        // For now it uses force_radar toggle from menu
        if self.force_radar {
            self.radar_active = true;
            self.amber_silver_active = false; // Example
        }
    }

    pub fn has_active_alert(&self) -> bool {
        self.radar_active || self.amber_silver_active || self.force_radar
    }
}