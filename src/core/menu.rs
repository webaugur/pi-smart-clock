use crate::drivers::platform::Platform;
use crate::drivers::rotary_encoder::RotaryEncoder;
use crate::layout::l;

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

    pub async fn update<P: Platform>(&mut self, _platform: &mut P, encoder: &mut RotaryEncoder) {
        self.close_requested = false;
        self.open_time_set = false;
        self.open_about = false;

        if encoder.button_pressed {
            match self.current {
                MenuState::Main => match self.selected {
                    0 => self.open_time_set = true,
                    1 => self.current = MenuState::Alerts,
                    2 => self.open_about = true,
                    3 => self.close_requested = true,
                    _ => {}
                },
                _ => self.current = MenuState::Main,
            }
            encoder.button_pressed = false;
        }

        if encoder.value != 0 {
            self.selected = (self.selected as i32 + encoder.value).rem_euclid(4) as usize;
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
        let layout = l();
        let cx = layout.screen_w / 2;
        let menu_y = if layout.screen_h > layout.screen_w {
            192
        } else {
            80
        };
        platform.clear_center_area().await;
        platform
            .draw_text("MENU", cx - 64, menu_y, 44, 0x00FFCC)
            .await;
        let items = ["Set Time", "Alerts", "About", "Back"];
        for (i, item) in items.iter().enumerate() {
            let y = menu_y + 96 + (i as i32 * 64);
            let color = if i == self.selected {
                0xFFFF00
            } else {
                0xAAAAAA
            };
            platform.draw_text(item, cx - 80, y, 32, color).await;
        }
    }
}