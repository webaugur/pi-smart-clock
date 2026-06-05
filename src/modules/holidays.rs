use crate::panel::Panel;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct HolidaysPanel {
    pub holidays: Vec<String>,
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
        let _ = canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
        canvas.set_draw_color(Color::RGB(200, 120, 80));
        let _ = canvas.fill_rect(Rect::new(x + 4, y + 4, (w - 8) as u32, 3));
    }

    fn update(&mut self) {}
}