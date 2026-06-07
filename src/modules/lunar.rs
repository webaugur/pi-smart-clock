use crate::icons::draw_symbolic_icon;
use crate::modules::bottom_module::{BottomModule, PanelLine};
use crate::modules::module_id::ModuleId;
use crate::panel::Panel;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Very small self-contained lunar phase calculator (mean synodic month).
/// Good enough for visualization and "current" phase until a full high-accuracy engine is added.
pub fn lunar_phase_fraction(days_since_known_new_moon: f64) -> f64 {
    let synodic = 29.530588853; // mean synodic month
    let frac = (days_since_known_new_moon % synodic) / synodic;
    if frac < 0.0 { frac + 1.0 } else { frac }
}

/// Approximate illumination (0.0 new .. 1.0 full).
pub fn illumination(phase: f64) -> f64 {
    // simple cosine model
    (1.0 - (phase * std::f64::consts::TAU).cos()) / 2.0
}

/// Returns a short phase name for display.
pub fn phase_name(phase: f64) -> &'static str {
    if phase < 0.03 || phase > 0.97 { "New Moon" }
    else if phase < 0.22 { "Waxing Crescent" }
    else if phase < 0.28 { "First Quarter" }
    else if phase < 0.47 { "Waxing Gibbous" }
    else if phase < 0.53 { "Full Moon" }
    else if phase < 0.72 { "Waning Gibbous" }
    else if phase < 0.78 { "Last Quarter" }
    else { "Waning Crescent" }
}

/// Very rough "Chinese New Year" approximation for demo / table fallback.
/// Real implementation will use the full lunar engine or pre-derived table.
pub fn approx_chinese_new_year_gregorian(year: i32) -> (u32, u32) {
    // Very approximate (real CNY is lunar 1/1). Used only as placeholder.
    // Known recent values for demo: 2024-02-10, 2025-01-29, 2026-02-17-ish.
    match year {
        2024 => (2, 10),
        2025 => (1, 29),
        2026 => (2, 17),
        2027 => (2, 6),
        _ => (2, 10), // fallback
    }
}

/// Lunar module (upper row).
/// Hosts full lunar calendar output + NASA HD lunar phase photo display (Linux).
/// Follows the exact same BottomModule / Panel contract.
pub struct LunarPanel {
    lines: Vec<String>,
    phase: f64,
}

impl LunarPanel {
    pub fn new() -> Self {
        // Demo: use a fixed offset so the phase moves slowly with real time.
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let known_new = 1_700_000_000.0_f64; // arbitrary recent new moon epoch
        let phase = lunar_phase_fraction((now - known_new) / 86400.0);
        let name = phase_name(phase);
        let illum = (illumination(phase) * 100.0) as u8;

        Self {
            phase,
            lines: vec![
                format!("Phase: {}", name),
                format!("Illum: {}%", illum),
                "Photo: (NASA HD)".to_string(),
            ],
        }
    }
}

impl BottomModule for LunarPanel {
    fn id(&self) -> ModuleId {
        ModuleId::Lunar
    }

    fn draw_background(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        self.draw(canvas, x, y, w, h);
    }

    fn title(&self) -> (String, u32) {
        ("Lunar".to_string(), 0x88FFCC)
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
        // Recompute phase slowly for demo (real version will use wall time + full engine)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let known_new = 1_700_000_000.0_f64;
        self.phase = lunar_phase_fraction((now - known_new) / 86400.0);
        let name = phase_name(self.phase);
        let illum = (illumination(self.phase) * 100.0) as u8;
        self.lines[0] = format!("Phase: {}", name);
        self.lines[1] = format!("Illum: {}%", illum);
    }
}

impl Panel for LunarPanel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        canvas.set_draw_color(Color::RGB(17, 17, 17));
        let _ = canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
        canvas.set_draw_color(Color::RGB(0x88, 0xFF, 0xCC));
        let _ = canvas.fill_rect(Rect::new(x + 4, y + 4, (w - 8) as u32, 3));

        // Simple phase disk (filled circle with "terminator" hint)
        let cx = x + w / 2 - 30;
        let cy = y + h / 2;
        let r = ((h - 30).max(30) as f32 * 0.4) as i32;
        canvas.set_draw_color(Color::RGB(200, 200, 220));
        let _ = canvas.fill_rect(Rect::new(cx - r, cy - r, (r * 2) as u32, (r * 2) as u32)); // cheap disk

        // NASA photo placeholder text (real version will blit the HD texture)
        canvas.set_draw_color(Color::RGB(0x88, 0xFF, 0xCC));
        // (texture blit for real NASA photo will go here in Lunar module)

        let icon_size = ((h - 20).max(80) as u32).min(112);
        let icon_x = x + w - icon_size as i32 - 6;
        let icon_y = y + (h - icon_size as i32) / 2;
        crate::icons::draw_icon(
            canvas,
            "status/weather-clear-night-symbolic.svg",
            icon_x,
            icon_y,
            icon_size,
        );
    }

    fn update(&mut self) {}
}

// Public helpers for other modules (Zodiac, Pentagram, CN holidays, etc.).
// They are already defined above as `pub fn`.
