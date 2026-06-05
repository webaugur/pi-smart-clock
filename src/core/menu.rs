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
    close_requested: bool,
    open_time_set: bool,
    open_about: bool,
}

impl MenuSystem {
    pub fn new() -> Self {
        Self {
            current: MenuState::Main,
            selected: 0,
            close_requested: false,
            open_time_set: false,
            open_about: false,
        }
    }

    pub async fn update<P: Platform>(&mut self, platform: &mut P, encoder: &mut RotaryEncoder) {
        self.close_requested = false;
        self.open_time_set = false;
        self.open_about = false;

        if encoder.button_pressed {
            match self.current {
                MenuState::Main => match self.selected {
                    0 => self.open_time_set = true,
                    1 => self.current = MenuState::Alerts,
                    2 => self.open_about = true,
                    _ => self.close_requested = true,
                },
                _ => self.current = MenuState::Main,
            }
            encoder.button_pressed = false;
        }

        if encoder.value != 0 {
            self.selected = (self.selected as i32 + encoder.value).rem_euclid(4).max(0) as usize;
            encoder.value = 0;
        }
    }

    pub fn should_open_time_set(&self) -> bool {
        self.open_time_set
    }

    pub fn should_open_about(&self) -> bool {
        self.open_about
    }

    pub fn should_close(&self) -> bool {
        self.close_requested
    }

    pub async fn draw<P: Platform>(&self, platform: &mut P) {
        platform.clear_center_area().await;
        platform.draw_text("MENU", 340, 60, 28, 0x00FFCC).await;
        let items = ["Set Time", "Alerts", "About", "Back"];
        for (i, item) in items.iter().enumerate() {
            let y = 120 + (i as i32 * 40);
            let color = if i == self.selected { 0xFFFF00 } else { 0xAAAAAA };
            platform.draw_text(item, 300, y, 20, color).await;
        }
    }
}
