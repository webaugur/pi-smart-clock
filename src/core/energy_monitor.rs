use crate::drivers::platform::Platform;

pub struct EnergyMonitor {
    pub current_power_w: f32,
    pub total_energy_kwh: f32,
    pub outlet_name: String,
}

impl EnergyMonitor {
    pub fn new() -> Self {
        Self {
            current_power_w: 0.0,
            total_energy_kwh: 0.0,
            outlet_name: "Living Room".to_string(),
        }
    }

    pub async fn update<P: Platform>(&mut self, platform: &mut P) {
        // TODO: Fetch from MQTT topic or HTTP API
        self.current_power_w = 42.7;
        self.total_energy_kwh += 0.001;
    }

    pub async fn draw<P: Platform>(&self, platform: &mut P, x: i32, y: i32, w: i32, h: i32) {
        platform.draw_rect(x, y, w, h, 0x1A2A3A);
        platform.draw_text("ENERGY", x + 20, y + 10, 14, 0x00FFAA);
        platform.draw_text(&format!("{:.1} W", self.current_power_w), x + 20, y + 40, 22, 0xFFFFFF);
        platform.draw_text(&format!("{:.2} kWh", self.total_energy_kwh), x + 20, y + 70, 14, 0xAAAAAA);
    }
}