use crate::drivers::platform::Platform;
use crate::core::alerts::AlertManager;

pub struct WeatherPanel {
    pub last_update: u64,
    pub radar_enabled: bool,
}

impl WeatherPanel {
    pub fn new() -> Self {
        Self {
            last_update: 0,
            radar_enabled: true,
        }
    }

    pub async fn update<P: Platform>(&mut self, platform: &mut P, alerts: &AlertManager) {
        // Smart scheduling: 3 hours normally, 5 minutes during alerts
        let interval = if alerts.radar_active || alerts.amber_silver_active {
            5 * 60 // 5 minutes
        } else {
            3 * 60 * 60 // 3 hours
        };

        println!("Weather panel updated (interval: {}s)", interval);
    }

    pub async fn draw_radar_overlay<P: Platform>(&self, platform: &mut P, alerts: &AlertManager) {
        if self.radar_enabled && (alerts.radar_active || alerts.amber_silver_active) {
            // Draw radar in bottom center panel
            platform.draw_rect(267, 320, 266, 160, 0x112244);
            platform.draw_text("WEATHER RADAR", 310, 370, 18, 0x00FFAA);
            platform.draw_text("Active Alert Overlay", 300, 400, 14, 0x88FF88);
        }
    }
}