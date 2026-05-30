use crate::panel::Panel;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct WeatherPanel {
    temp: i32,
    condition: String,
}

impl WeatherPanel {
    pub fn new() -> Self {
        Self {
            temp: 72,
            condition: "Partly Cloudy".to_string(),
        }
    }
}

impl Panel for WeatherPanel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        canvas.set_draw_color(Color::RGB(30, 45, 30));
    }

    fn update(&mut self) {}
}