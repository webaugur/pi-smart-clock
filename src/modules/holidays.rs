use crate::panel::Panel;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct HolidaysPanel {
    holidays: Vec<String>,
}

impl HolidaysPanel {
    pub fn new() -> Self {
        Self {
            holidays: vec![
                "Jun 19 - Juneteenth".to_string(),
                "Jul 4 - Independence Day".to_string(),
            ],
        }
    }
}

impl Panel for HolidaysPanel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        canvas.set_draw_color(Color::RGB(45, 30, 30));
    }

    fn update(&mut self) {}
}