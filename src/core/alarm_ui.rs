use crate::core::alarm::Alarm;
use crate::drivers::platform::Platform;
use crate::layout::{CENTER_X, CENTER_Y};

/// Alarm overlay text (video plays underneath in center panel).
pub struct AlarmUI {
    pub active: bool,
    pub current_alarm: Option<Alarm>,
}

impl AlarmUI {
    pub fn new() -> Self {
        Self {
            active: false,
            current_alarm: None,
        }
    }

    pub async fn show<P: Platform>(&mut self, platform: &mut P, alarm: &Alarm) {
        self.active = true;
        self.current_alarm = Some(alarm.clone());
        self.draw_overlay(platform, alarm).await;
    }

    pub async fn draw_overlay<P: Platform>(&mut self, platform: &mut P, alarm: &Alarm) {
        let header = if alarm.label.contains("Amber") || alarm.label.contains("Silver") {
            "ALERT"
        } else {
            "ALARM"
        };

        platform
            .draw_text(header, CENTER_X + 10, CENTER_Y - 20, 28, 0xFF4444)
            .await;
        platform
            .draw_text(
                &format!("{:02}:{:02}", alarm.hour, alarm.minute),
                CENTER_X + 70,
                CENTER_Y + 170,
                36,
                0xFFFF88,
            )
            .await;
        platform
            .draw_text(&alarm.label, CENTER_X + 30, CENTER_Y + 210, 18, 0xCCCCCC)
            .await;
        platform
            .draw_text("Button = Dismiss", CENTER_X + 40, CENTER_Y + 240, 14, 0x88FF88)
            .await;
    }

    pub async fn hide<P: Platform>(&mut self, _platform: &mut P) {
        self.active = false;
        self.current_alarm = None;
    }
}