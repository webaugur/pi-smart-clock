use crate::core::alarm::Alarm;
use crate::drivers::platform::Platform;

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

        platform.draw_text(header, 280, 125, 28, 0xFF4444).await;
        platform
            .draw_text(
                &format!("{:02}:{:02}", alarm.hour, alarm.minute),
                340,
                305,
                36,
                0xFFFF88,
            )
            .await;
        platform.draw_text(&alarm.label, 300, 345, 18, 0xCCCCCC).await;
        platform
            .draw_text("Button = Dismiss", 290, 375, 14, 0x88FF88)
            .await;
    }

    pub async fn hide<P: Platform>(&mut self, _platform: &mut P) {
        self.active = false;
        self.current_alarm = None;
    }
}