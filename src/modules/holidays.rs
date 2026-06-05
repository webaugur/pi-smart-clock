use crate::icons::draw_symbolic_icon;
use crate::modules::bottom_module::{BottomModule, PanelLine};
use crate::modules::module_id::ModuleId;
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
                "Sep 1 - Labor Day".to_string(),
            ],
        }
    }
}

impl BottomModule for HolidaysPanel {
    fn id(&self) -> ModuleId {
        ModuleId::Holidays
    }

    fn draw_background(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        self.draw(canvas, x, y, w, h);
    }

    fn title(&self) -> (String, u32) {
        ("Holidays".to_string(), 0xFFAA88)
    }

    fn lines(&self) -> Vec<PanelLine> {
        self.holidays
            .iter()
            .take(3)
            .map(|h| PanelLine {
                text: h.clone(),
                size_pt: 0,
            })
            .collect()
    }
}

impl Panel for HolidaysPanel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        canvas.set_draw_color(Color::RGB(17, 17, 17));
        let _ = canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
        canvas.set_draw_color(Color::RGB(200, 120, 80));
        let _ = canvas.fill_rect(Rect::new(x + 4, y + 4, (w - 8) as u32, 3));

        let icon_size = ((h - 20).max(80) as u32).min(112);
        let icon_x = x + w - icon_size as i32 - 6;
        let icon_y = y + (h - icon_size as i32) / 2;
        draw_symbolic_icon(
            canvas,
            "status/starred-symbolic.svg",
            icon_x,
            icon_y,
            icon_size,
            Color::RGB(255, 170, 136),
        );
    }

    fn update(&mut self) {}
}