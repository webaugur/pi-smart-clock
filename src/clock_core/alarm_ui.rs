use crate::clock_core::alarm::Alarm;
use crate::drivers::platform::Platform;
use crate::layout::l;

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
        let layout = l();
        let header = if alarm.label.contains("Amber") || alarm.label.contains("Silver") {
            "ALERT"
        } else {
            "ALARM"
        };

        platform
            .draw_text(
                header,
                layout.center_x + 16,
                layout.center_y - 32,
                44,
                0xFF4444,
            )
            .await;
        platform
            .draw_text(
                &format!("{:02}:{:02}", alarm.hour, alarm.minute),
                layout.center_x + 112,
                layout.center_y + layout.center_h as i32 - 48,
                58,
                0xFFFF88,
            )
            .await;
        platform
            .draw_text(
                &alarm.label,
                layout.center_x + 48,
                layout.center_y + layout.center_h as i32 - 8,
                28,
                0xCCCCCC,
            )
            .await;
        platform
            .draw_text(
                "Button = Dismiss",
                layout.center_x + 64,
                layout.center_y + layout.center_h as i32 + 24,
                22,
                0x88FF88,
            )
            .await;
    }

    pub async fn hide<P: Platform>(&mut self, _platform: &mut P) {
        self.active = false;
        self.current_alarm = None;
    }
}