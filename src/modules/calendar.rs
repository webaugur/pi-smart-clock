use crate::panel::Panel;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct CalendarPanel {
    pub events: Vec<String>,
}

impl CalendarPanel {
    pub fn new() -> Self {
        Self {
            events: vec![
                "09:00 Team Sync".to_string(),
                "11:30 Doctor".to_string(),
                "15:00 Project Review".to_string(),
            ],
        }
    }
}

impl Panel for CalendarPanel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        canvas.set_draw_color(Color::RGB(25, 25, 45));
        let _ = canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
        canvas.set_draw_color(Color::RGB(80, 120, 200));
        let _ = canvas.fill_rect(Rect::new(x + 4, y + 4, (w - 8) as u32, 3));
    }

    fn update(&mut self) {}
}