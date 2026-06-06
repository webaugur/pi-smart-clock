use crate::drivers::platform::Platform;
#[cfg(not(feature = "linux-full"))]
use crate::prelude::*;

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

    pub async fn update_from_mqtt<P: Platform>(
        &mut self,
        _platform: &mut P,
        topic: &str,
        payload: &str,
    ) {
        if topic.contains("power") {
            if let Ok(power) = payload.parse::<f32>() {
                self.current_power_w = power;
            }
        }
        if topic.contains("energy") {
            if let Ok(energy) = payload.parse::<f32>() {
                self.total_energy_kwh = energy;
            }
        }
    }

    pub async fn draw<P: Platform>(&self, platform: &mut P, x: i32, y: i32) {
        platform
            .draw_text(
                &format!("Power: {:.1} W", self.current_power_w),
                x,
                y,
                16,
                0x00FFAA,
            )
            .await;
        platform
            .draw_text(
                &format!("Energy: {:.2} kWh", self.total_energy_kwh),
                x,
                y + 25,
                16,
                0x00FFAA,
            )
            .await;
    }
}