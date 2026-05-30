use crate::drivers::platform::Platform;

pub struct RotaryEncoder {
    pub value: i32,
    pub button_pressed: bool,
    pub long_press: bool,
}

impl RotaryEncoder {
    pub fn new() -> Self {
        Self { value: 0, button_pressed: false, long_press: false }
    }

    pub async fn update<P: Platform>(&mut self, platform: &mut P) {
        let delta = platform.read_rotary_delta();
        self.value += delta;

        if platform.read_pushbutton() {
            self.button_pressed = true;
        }
    }
}