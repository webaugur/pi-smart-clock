use crate::core::alerts::AlertManager;
use crate::drivers::platform::Platform;

pub struct WeatherRadarPanel {
    pub visible: bool,
}

impl WeatherRadarPanel {
    pub fn new() -> Self {
        Self { visible: false }
    }

    pub async fn update<P: Platform>(&mut self, platform: &mut P, alerts: &AlertManager) {
        self.visible = alerts.radar_active || alerts.force_radar;
        if self.visible {
            platform.draw_rect(267, 320, 266, 160, 0x112244).await;
            platform
                .draw_text("WEATHER RADAR", 310, 370, 18, 0x00FFAA)
                .await;
            platform
                .draw_text("Active Alert Overlay", 300, 400, 14, 0x88FF88)
                .await;
        }
    }
}