use crate::drivers::platform::Platform;

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

    pub async fn check<P: Platform>(&mut self, platform: &mut P) {
        // Check current time against alarms
    }
}