use crate::drivers::platform::Platform;
use crate::drivers::rotary_encoder::RotaryEncoder;
use crate::layout::l;
pub struct TimeSetUI {
    pub editing: bool,
    hour: u32,
    minute: u32,
    selected_field: u8,
}

impl TimeSetUI {
    pub fn new() -> Self {
        Self {
            editing: false,
            hour: 12,
            minute: 0,
            selected_field: 0,
        }
    }

    pub async fn update<P: Platform>(&mut self, platform: &mut P, encoder: &mut RotaryEncoder) {
        if encoder.button_pressed {
            if !self.editing {
                self.editing = true;
            } else {
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
            let layout = l();
            let cx = layout.screen_w / 2;
            platform.clear_center_area().await;
            platform
                .draw_text(
                    &format!("{:02}:{:02}", self.hour, self.minute),
                    cx - 80,
                    layout.center_y + 32,
                    76,
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
                    cx - 64,
                    layout.center_y + 128,
                    28,
                    0x88CCFF,
                )
                .await;
        }
    }
}