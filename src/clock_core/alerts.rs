use crate::drivers::platform::Platform;
use crate::prelude::*;
use crate::timing::{Duration, Instant};

pub struct AlertManager {
    pub same_codes: [String; 9],
    pub radar_active: bool,
    pub amber_silver_active: bool,
    pub force_radar: bool,
    last_check: Option<Instant>,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            same_codes: Default::default(),
            radar_active: false,
            amber_silver_active: false,
            force_radar: false,
            last_check: None,
        }
    }

    pub async fn check_nws_alerts<P: Platform>(&mut self, _platform: &mut P) {
        let now = Instant::now();
        if let Some(last) = self.last_check {
            if now.duration_since(last) < Duration::from_secs(60) {
                return;
            }
        }
        self.last_check = Some(now);

        if self.force_radar {
            self.radar_active = true;
            self.amber_silver_active = false;
        }
    }

    pub fn has_active_alert(&self) -> bool {
        self.radar_active || self.amber_silver_active || self.force_radar
    }
}