use chrono::{Local, Timelike};

use crate::drivers::platform::Platform;

const TICK: &str = "sounds/tick.wav";
const TOCK: &str = "sounds/tock.wav";
const QUARTER: &str = "sounds/quarter.wav";
const HALF: &str = "sounds/half.wav";
const HOUR: &str = "sounds/bell.wav";

/// Quarter / half / hour chimes driven by wall-clock time.
pub struct ChimeEngine {
    last_second: u32,
    last_quarter: u32,
}

impl ChimeEngine {
    pub fn new() -> Self {
        Self {
            last_second: 255,
            last_quarter: 255,
        }
    }

    pub async fn tick<P: Platform>(&mut self, platform: &mut P) {
        let now = Local::now();
        let sec = now.second();
        if sec == self.last_second {
            return;
        }
        self.last_second = sec;

        if sec % 2 == 0 {
            platform.play_sound(TICK, 1.0).await;
        } else {
            platform.play_sound(TOCK, 1.0).await;
        }

        let quarter_idx = now.minute() / 15;
        if quarter_idx != self.last_quarter && now.minute() % 15 == 0 && now.second() == 0 {
            self.last_quarter = quarter_idx;
            if now.minute() == 0 {
                platform.play_sound(HOUR, 1.0).await;
            } else if now.minute() == 30 {
                platform.play_sound(HALF, 1.0).await;
            } else {
                platform.play_sound(QUARTER, 1.0).await;
            }
        }
    }
}
