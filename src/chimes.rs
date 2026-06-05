use chrono::{Local, Timelike};

use crate::platform::linux_audio::{ChimeKind, LinuxAudioEngine};

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

    pub fn tick(&mut self, audio: &mut LinuxAudioEngine) {
        let now = Local::now();
        let sec = now.second();
        if sec == self.last_second {
            return;
        }
        self.last_second = sec;

        let quarter_chime = now.minute() % 15 == 0 && sec == 0;
        let mut played_quarter = false;

        if quarter_chime {
            let quarter_idx = now.minute() / 15;
            if quarter_idx != self.last_quarter {
                self.last_quarter = quarter_idx;
                played_quarter = true;
                if now.minute() == 0 {
                    audio.play_chime(ChimeKind::Hour);
                } else if now.minute() == 30 {
                    audio.play_chime(ChimeKind::Half);
                } else {
                    audio.play_chime(ChimeKind::Quarter);
                }
            }
        }

        if !played_quarter {
            if sec % 2 == 0 {
                audio.play_chime(ChimeKind::Tick);
            } else {
                audio.play_chime(ChimeKind::Tock);
            }
        }
    }
}