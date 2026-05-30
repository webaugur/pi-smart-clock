use crate::drivers::platform::Platform;
use crate::core::alarm::Alarm;

pub async fn show_alarm_preview<P: Platform>(platform: &mut P, alarm: &Alarm) {
    platform.clear_center_area();
    platform.draw_text("\u23f0 ALARM", 300, 80, 36, 0xFF4444);
    platform.draw_text(&format!("{:02}:{:02}", alarm.hour, alarm.minute), 340, 160, 52, 0xFFFF88);
    platform.draw_text(&alarm.label, 320, 230, 22, 0xCCCCCC);
    platform.draw_text("Press Button = Snooze 9 min", 220, 300, 16, 0x88FF88);
    platform.draw_text("Hold Button = Dismiss", 250, 330, 16, 0xFF8888);
}