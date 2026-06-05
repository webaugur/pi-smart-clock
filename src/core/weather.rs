use crate::core::alerts::AlertManager;
use crate::drivers::platform::Platform;

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

    pub async fn update<P: Platform>(&mut self, _platform: &mut P, alerts: &AlertManager) {
        let interval = if alerts.radar_active || alerts.amber_silver_active {
            5 * 60
        } else {
            3 * 60 * 60
        };
        self.last_update = interval;
    }

    pub async fn draw_radar_overlay<P: Platform>(
        &self,
        platform: &mut P,
        alerts: &AlertManager,
    ) {
        if self.radar_enabled && (alerts.radar_active || alerts.amber_silver_active) {
            platform
                .draw_rect(
                    crate::layout::CENTER_X,
                    crate::layout::BOTTOM_Y,
                    crate::layout::CENTER_W as i32,
                    crate::layout::BOTTOM_H as i32,
                    0x112244,
                )
                .await;
            platform
                .draw_text(
                    "WEATHER RADAR",
                    crate::layout::CENTER_X + 32,
                    crate::layout::BOTTOM_Y + 128,
                    28,
                    0x00FFAA,
                )
                .await;
            platform
                .draw_text(
                    "Active Alert Overlay",
                    crate::layout::CENTER_X + 16,
                    crate::layout::BOTTOM_Y + 176,
                    22,
                    0x88FF88,
                )
                .await;
        }
    }
}