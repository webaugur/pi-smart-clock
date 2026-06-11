use crate::modules::bottom_module::{BottomModule, PanelLine};
use crate::modules::module_id::ModuleId;
use crate::panel::Panel;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// VenusMoonPentagram module (upper row).
/// Visualizes the current Venus-Moon pentagram cycle.
/// Uses lunar engine + Venus synodic data to draw the characteristic 5-pointed geometry + current position.
/// Same BottomModule / Panel format.
pub struct VenusMoonPentagramPanel {
    lines: Vec<String>,
}

impl VenusMoonPentagramPanel {
    pub fn new() -> Self {
        Self {
            lines: vec![
                "Venus-Moon".to_string(),
                "Pentagram".to_string(),
                "Cycle".to_string(),
            ],
        }
    }

    // Approximate position in the famous ~8-year (5 synodic Venus) pentagram cycle.
    fn cycle_fraction(&self) -> f64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        // 5 Venus synodic periods ≈ 8 years
        let cycle_days = 5.0 * 583.92;
        let epoch = 1_600_000_000.0;
        ((now - epoch) / 86400.0 % cycle_days) / cycle_days
    }
}

impl BottomModule for VenusMoonPentagramPanel {
    fn id(&self) -> ModuleId {
        ModuleId::Pentagram
    }

    fn draw_background(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        self.draw(canvas, x, y, w, h);
    }

    fn title(&self) -> (String, u32) {
        ("Pentagram".to_string(), 0xFFAA88)
    }

    fn lines(&self) -> Vec<PanelLine> {
        self.lines
            .iter()
            .take(3)
            .map(|l| PanelLine {
                text: l.clone(),
                size_pt: 0,
            })
            .collect()
    }

    fn tick(&mut self, _alerts_active: bool) {
        let f = self.cycle_fraction();
        self.lines[2] = format!("Pos: {:.0}%", f * 100.0);
    }
}

impl Panel for VenusMoonPentagramPanel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        canvas.set_draw_color(Color::RGB(17, 17, 17));
        let _ = canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
        canvas.set_draw_color(Color::RGB(0xFF, 0xAA, 0x88));
        let _ = canvas.fill_rect(Rect::new(x + 4, y + 4, (w - 8) as u32, 3));

        // Draw a proper 5-point pentagram (Venus pentagram) + current position marker.
        let cx = x + w / 2;
        let cy = y + h / 2;
        let r = (h.min(w) as f32 * 0.38) as i32;
        let f = self.cycle_fraction(); // 0..1 position in the cycle

        // 5 vertices of the pentagram (classic 72° star)
        let mut pts: Vec<(i32, i32)> = Vec::new();
        for i in 0..5 {
            let angle = (i as f32 * 144.0 - 90.0).to_radians(); // 144° step for star
            let px = cx as f32 + (r as f32) * angle.cos();
            let py = cy as f32 + (r as f32) * angle.sin();
            pts.push((px as i32, py as i32));
        }

        canvas.set_draw_color(Color::RGB(0xFF, 0xAA, 0x88));
        for i in 0..5 {
            let (x1, y1) = pts[i];
            let (x2, y2) = pts[(i + 2) % 5]; // connect every 2nd vertex for the star
            let _ = canvas.draw_line(
                sdl2::rect::Point::new(x1, y1),
                sdl2::rect::Point::new(x2, y2),
            );
        }

        // Current position marker on the cycle (moves around the pentagram)
        let idx = ((f * 5.0) as usize) % 5;
        let t = (f * 5.0).fract();
        let (x1, y1) = pts[idx];
        let (x2, y2) = pts[(idx + 2) % 5];
        let mx = x1 + ((x2 - x1) as f32 * t as f32) as i32;
        let my = y1 + ((y2 - y1) as f32 * t as f32) as i32;

        canvas.set_draw_color(Color::RGB(255, 220, 120));
        let _ = canvas.fill_rect(Rect::new(mx - 4, my - 4, 8, 8)); // bright dot for current position

        let icon_size = ((h - 20).max(80) as u32).min(112);
        let icon_x = x + w - icon_size as i32 - 6;
        let icon_y = y + (h - icon_size as i32) / 2;
        crate::icons::draw_icon(
            canvas,
            "status/starred-symbolic.svg",
            icon_x,
            icon_y,
            icon_size,
        );
    }

    fn update(&mut self) {}
}
