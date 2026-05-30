use crate::drivers::platform::Platform;
use crate::drivers::rotary_encoder::RotaryEncoder;

pub enum MenuState {
    Main,
    Alarms,
    SetTime,
    Alerts,
    Settings,
}

pub struct MenuSystem {
    pub current: MenuState,
    pub selected: usize,
}

impl MenuSystem {
    pub fn new() -> Self {
        Self {
            current: MenuState::Main,
            selected: 0,
        }
    }

    pub async fn update<P: Platform>(&mut self, platform: &mut P, encoder: &mut RotaryEncoder) {
        if encoder.button_pressed {
            // Handle selection
            encoder.button_pressed = false;
        }

        if encoder.value != 0 {
            self.selected = (self.selected as i32 + encoder.value).rem_euclid(5) as usize;
            encoder.value = 0;
        }
    }

    pub async fn draw<P: Platform>(&self, platform: &mut P) {
        platform.clear_center_area();
        platform.draw_text("MENU", 340, 60, 28, 0x00FFCC);
        // Draw menu items based on current state
    }
}