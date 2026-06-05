use crate::drivers::platform::Platform;
use chrono::{Local, Timelike};

#[derive(Clone)]
pub struct Alarm {
    pub id: usize,
    pub hour: u32,
    pub minute: u32,
    pub enabled: bool,
    pub repeat: bool,
    pub label: String,
    pub sound_file: String,
    pub snooze_minutes: u32,
}

pub struct AlarmManager {
    pub alarms: [Option<Alarm>; 4],
}

impl AlarmManager {
    pub fn new() -> Self {
        Self { alarms: [None, None, None, None] }
    }

    pub async fn check<P: Platform>(&mut self, platform: &mut P, ringing: &mut Option<usize>) {
        let now = Local::now();
        let h = now.hour();
        let m = now.minute();
        let sec = now.second();
        if sec > 2 {
            return;
        }
        for (i, slot) in self.alarms.iter().enumerate() {
            if let Some(a) = slot {
                if a.enabled && a.hour == h && a.minute == m {
                    *ringing = Some(i);
                    platform.play_sound(&a.sound_file, 0.9).await;
                }
            }
        }
    }
}
