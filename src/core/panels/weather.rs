use crate::core::alerts::AlertManager;
use crate::drivers::platform::Platform;
use crate::layout::{l, Orientation};

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
            let layout = l();
            let radar_x = if layout.orientation == Orientation::Landscape {
                layout.cal_w
            } else {
                layout.center_x
            };
            let radar_w = if layout.orientation == Orientation::Landscape {
                layout.screen_w - layout.cal_w - layout.hol_w
            } else {
                layout.center_w as i32
            };
            platform
                .draw_rect(radar_x, layout.bottom_y, radar_w, layout.bottom_h, 0x112244)
                .await;
            platform
                .draw_text(
                    "WEATHER RADAR",
                    radar_x + 32,
                    layout.bottom_y + layout.bottom_h / 3,
                    28,
                    0x00FFAA,
                )
                .await;
            platform
                .draw_text(
                    "Active Alert Overlay",
                    radar_x + 16,
                    layout.bottom_y + layout.bottom_h / 2,
                    22,
                    0x88FF88,
                )
                .await;
        }
    }
}