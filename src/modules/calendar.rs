use crate::panel::Panel;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct CalendarPanel {
    events: Vec<String>,
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
    }

    fn update(&mut self) {}
}