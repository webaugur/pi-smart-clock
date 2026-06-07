use crate::icons::draw_symbolic_icon;
use crate::modules::bottom_module::{BottomModule, PanelLine};
use crate::modules::module_id::ModuleId;
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

impl BottomModule for CalendarPanel {
    fn id(&self) -> ModuleId {
        ModuleId::Calendar
    }

    fn draw_background(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        self.draw(canvas, x, y, w, h);
    }

    fn title(&self) -> (String, u32) {
        ("Calendar".to_string(), 0x88AAFF)
    }

    fn lines(&self) -> Vec<PanelLine> {
        self.events
            .iter()
            .take(3)
            .map(|ev| PanelLine {
                text: ev.clone(),
                size_pt: 0,
            })
            .collect()
    }
}

impl Panel for CalendarPanel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        canvas.set_draw_color(Color::RGB(17, 17, 17));
        let _ = canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
        canvas.set_draw_color(Color::RGB(80, 120, 200));
        let _ = canvas.fill_rect(Rect::new(x + 4, y + 4, (w - 8) as u32, 3));

        let icon_size = ((h - 20).max(80) as u32).min(112);
        let icon_x = x + w - icon_size as i32 - 6;
        let icon_y = y + (h - icon_size as i32) / 2;
        crate::icons::draw_icon(
            canvas,
            "apps/calendar-symbolic.svg",
            icon_x,
            icon_y,
            icon_size,
        );
    }

    fn update(&mut self) {}
}