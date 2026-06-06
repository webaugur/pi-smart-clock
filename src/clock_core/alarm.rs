use crate::drivers::platform::Platform;
#[cfg(not(feature = "linux-full"))]
use crate::prelude::*;

#[derive(Clone)]
pub struct Alarm {
    pub id: usize,
    pub hour: u32,
    pub minute: u32,
    pub enabled: bool,
    pub repeat: bool,
    pub label: String,
    pub sound_file: String,
    pub video_file: String,
    pub snooze_minutes: u32,
}

pub struct AlarmManager {
    pub alarms: [Option<Alarm>; 4],
    last_rung: Option<(u32, u32, usize)>,
}

impl AlarmManager {
    pub fn new() -> Self {
        Self {
            alarms: [None, None, None, None],
            last_rung: None,
        }
    }

    pub async fn check<P: Platform>(&mut self, platform: &mut P, ringing: &mut Option<usize>) {
        let (h, m, sec) = current_hms(platform);
        if sec > 2 {
            return;
        }

        for (i, slot) in self.alarms.iter().enumerate() {
            let Some(a) = slot else {
                continue;
            };
            if !a.enabled || a.hour != h || a.minute != m {
                continue;
            }
            if self.last_rung == Some((h, m, i)) {
                continue;
            }
            self.last_rung = Some((h, m, i));
            *ringing = Some(i);
            platform.play_alarm_loop(&a.sound_file).await;
        }
    }

    pub fn on_new_minute(&mut self, hour: u32, minute: u32) {
        if self.last_rung.map(|(h, m, _)| (h, m)) == Some((hour, minute)) {
            self.last_rung = None;
        }
    }
}

fn current_hms<P: Platform>(platform: &P) -> (u32, u32, u32) {
    #[cfg(feature = "linux-full")]
    {
        use chrono::Timelike;
        let now = platform.get_current_time();
        return (now.hour(), now.minute(), now.second());
    }
    #[cfg(not(feature = "linux-full"))]
    {
        let now = platform.get_current_time();
        (now.hour, now.minute, now.second)
    }
}