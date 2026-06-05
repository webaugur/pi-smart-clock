use crate::panel::Panel;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
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

    pub fn set_weather(&mut self, temp: i32, condition: String) {
        self.temp = temp;
        self.condition = condition;
    }

    pub fn temp(&self) -> i32 {
        self.temp
    }

    pub fn condition(&self) -> &str {
        &self.condition
    }
}

impl Panel for WeatherPanel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        canvas.set_draw_color(Color::RGB(20, 35, 55));
        let _ = canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
        canvas.set_draw_color(Color::RGB(0, 255, 170));
        let _ = canvas.fill_rect(Rect::new(x + 4, y + 4, (w - 8) as u32, 4));
    }

    fn update(&mut self) {}
}
