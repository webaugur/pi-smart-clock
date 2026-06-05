use crate::core::alerts::AlertManager;
use crate::drivers::platform::Platform;
use crate::layout::{BOTTOM_H, BOTTOM_Y, CENTER_W, CENTER_X};

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
            platform
                .draw_rect(
                    CENTER_X,
                    BOTTOM_Y,
                    CENTER_W as i32,
                    BOTTOM_H as i32,
                    0x112244,
                )
                .await;
            platform
                .draw_text(
                    "WEATHER RADAR",
                    CENTER_X + 20,
                    BOTTOM_Y + 80,
                    18,
                    0x00FFAA,
                )
                .await;
            platform
                .draw_text(
                    "Active Alert Overlay",
                    CENTER_X + 10,
                    BOTTOM_Y + 110,
                    14,
                    0x88FF88,
                )
                .await;
        }
    }
}