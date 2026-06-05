use std::time::{Duration, Instant};

use crate::core::alerts::AlertManager;
use crate::drivers::platform::Platform;

pub struct UpdateScheduler {
    last_weather: Instant,
    last_non_weather: Instant,
    last_alert_announce: Instant,
}

impl UpdateScheduler {
    pub fn new() -> Self {
        Self {
            last_weather: Instant::now() - Duration::from_secs(3 * 60 * 60),
            last_non_weather: Instant::now(),
            last_alert_announce: Instant::now(),
        }
    }

    pub async fn tick<P: Platform>(
        &mut self,
        platform: &mut P,
        alert_manager: &AlertManager,
    ) -> Option<(i32, String)> {
        let now = Instant::now();
        let mut weather_update = None;

        let weather_interval = if alert_manager.radar_active || alert_manager.amber_silver_active {
            Duration::from_secs(5 * 60)
        } else {
            Duration::from_secs(3 * 60 * 60)
        };

        if now.duration_since(self.last_weather) > weather_interval {
            if let Ok(data) = platform.fetch_weather().await {
                weather_update = Some(data);
            }
            self.last_weather = now;
        }

        if now.duration_since(self.last_non_weather) > Duration::from_secs(3600) {
            let _ = platform.fetch_calendar().await;
            let _ = platform.fetch_holidays().await;
            self.last_non_weather = now;
        }

        if alert_manager.has_active_alert()
            && now.duration_since(self.last_alert_announce) > Duration::from_secs(15 * 60)
        {
            platform.speak("Weather alert active").await;
            self.last_alert_announce = now;
        }

        weather_update
    }
}
