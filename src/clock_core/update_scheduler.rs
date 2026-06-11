use crate::timing::{Duration, Instant};
#[cfg(not(feature = "full"))]
use crate::prelude::*;

use crate::clock_core::alerts::AlertManager;
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

        let weather_interval = if alert_manager.radar_active || alert_manager.amber_silver_active {
            Duration::from_secs(5 * 60)
        } else {
            Duration::from_secs(3 * 60 * 60)
        };

        let weather_update = if now.duration_since(self.last_weather) > weather_interval {
            self.last_weather = now;
            weather_fetch(platform).await
        } else {
            None
        };

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

async fn weather_fetch<P: Platform>(platform: &mut P) -> Option<(i32, String)> {
    #[cfg(feature = "full")]
    {
        let _ = platform;
        None
    }
    #[cfg(not(feature = "full"))]
    {
        platform.fetch_weather().await.ok()
    }
}
