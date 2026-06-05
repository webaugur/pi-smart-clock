use crate::drivers::platform::Platform;
use crate::drivers::rotary_encoder::RotaryEncoder;
use crate::layout::SCREEN_W;
use chrono::{Local, Timelike};

pub struct TimeSetUI {
    pub editing: bool,
    hour: u32,
    minute: u32,
    selected_field: u8, // 0 = hour, 1 = minute
}

impl TimeSetUI {
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            editing: false,
            hour: now.hour(),
            minute: now.minute(),
            selected_field: 0,
        }
    }

    pub async fn update<P: Platform>(&mut self, platform: &mut P, encoder: &mut RotaryEncoder) {
        if encoder.button_pressed {
            if !self.editing {
                self.editing = true;
            } else {
                let _hour = self.hour;
                let _minute = self.minute;
                self.editing = false;
            }
            encoder.button_pressed = false;
        }

        if self.editing && encoder.value != 0 {
            if self.selected_field == 0 {
                self.hour = ((self.hour as i32 + encoder.value) % 24).max(0) as u32;
            } else {
                self.minute = ((self.minute as i32 + encoder.value * 5) % 60).max(0) as u32;
            }
            encoder.value = 0;
        }

        if self.editing {
            platform.clear_center_area().await;
            platform
                .draw_text(
                    &format!("{:02}:{:02}", self.hour, self.minute),
                    SCREEN_W / 2 - 50,
                    320,
                    48,
                    0xFFFF00,
                )
                .await;
            platform
                .draw_text(
                    if self.selected_field == 0 {
                        "↑ Hour"
                    } else {
                        "↑ Minute"
                    },
                    SCREEN_W / 2 - 40,
                    380,
                    18,
                    0x88CCFF,
                )
                .await;
        }
    }
}