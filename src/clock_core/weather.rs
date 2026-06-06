use crate::clock_core::alerts::AlertManager;
use crate::drivers::platform::Platform;
use crate::layout::l;

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
            let layout = l();
            let radar_x = if layout.orientation == crate::layout::Orientation::Landscape {
                layout.cal_w
            } else {
                0
            };
            let radar_w = if layout.orientation == crate::layout::Orientation::Landscape {
                layout.screen_w - layout.cal_w - layout.hol_w
            } else {
                layout.screen_w
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