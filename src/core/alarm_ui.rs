use crate::drivers::platform::Platform;
use crate::core::alarm::Alarm;

/// Visual Alarm Preview / Snooze UI
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

        platform.clear_center_area().await;

        let header = if alarm.label.contains("Amber") || alarm.label.contains("Silver") {
            "⚠️ ALERT ACTIVE"
        } else {
            "⏰ ALARM"
        };

        platform.draw_text(header, 280, 80, 36, 0xFF4444).await;
        platform.draw_text(&format!("{:02}:{:02}", alarm.hour, alarm.minute), 340, 160, 52, 0xFFFF88).await;
        platform.draw_text(&alarm.label, 300, 230, 22, 0xCCCCCC).await;
        platform.draw_text("Button = Snooze 9 min", 220, 300, 16, 0x88FF88).await;
        platform.draw_text("Hold Button = Dismiss", 250, 330, 16, 0xFF8888).await;
    }

    pub async fn hide<P: Platform>(&mut self, platform: &mut P) {
        self.active = false;
        self.current_alarm = None;
        platform.clear_center_area().await;
    }
}