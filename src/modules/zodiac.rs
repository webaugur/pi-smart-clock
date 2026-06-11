use crate::modules::bottom_module::{BottomModule, PanelLine};
use crate::modules::module_id::ModuleId;
use crate::panel::Panel;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone, Copy)]
struct ZodiacSign {
    name: &'static str,
    symbol: &'static str,
    slug: &'static str, // for zodiac-xxx-symbolic.svg
}

/// The 12 signs in order. Used for display and cycle.
const SIGNS: [ZodiacSign; 12] = [
    ZodiacSign { name: "Aries",       symbol: "♈", slug: "aries" },
    ZodiacSign { name: "Taurus",      symbol: "♉", slug: "taurus" },
    ZodiacSign { name: "Gemini",      symbol: "♊", slug: "gemini" },
    ZodiacSign { name: "Cancer",      symbol: "♋", slug: "cancer" },
    ZodiacSign { name: "Leo",         symbol: "♌", slug: "leo" },
    ZodiacSign { name: "Virgo",       symbol: "♍", slug: "virgo" },
    ZodiacSign { name: "Libra",       symbol: "♎", slug: "libra" },
    ZodiacSign { name: "Scorpio",     symbol: "♏", slug: "scorpio" },
    ZodiacSign { name: "Sagittarius", symbol: "♐", slug: "sagittarius" },
    ZodiacSign { name: "Capricorn",   symbol: "♑", slug: "capricorn" },
    ZodiacSign { name: "Aquarius",    symbol: "♒", slug: "aquarius" },
    ZodiacSign { name: "Pisces",      symbol: "♓", slug: "pisces" },
];

fn current_zodiac_sign() -> &'static ZodiacSign {
    // Simple seasonal approximation for "current" sign (modern tropical zodiac).
    // For production, a more accurate sun longitude calculation can be used.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Rough day of year
    let day_of_year = ((now / 86400) % 365) as u32;
    let idx = (day_of_year / 30) % 12; // ~30 days per sign
    &SIGNS[idx as usize]
}

/// Zodiac module (upper row).
/// Modern zodiac symbols (using playful cartoony icons) + Venus and synodic cycle information for the planets.
/// Same BottomModule / Panel format as all other modules.
pub struct ZodiacPanel {
    lines: Vec<String>,
    current: &'static ZodiacSign,
}

impl ZodiacPanel {
    pub fn new() -> Self {
        let current = current_zodiac_sign();
        let f = Self::venus_cycle_fraction();
        Self {
            current,
            lines: vec![
                format!("{} {}", current.symbol, current.name),
                format!("Venus: {:.0}%", f * 100.0),
                "Cycle info".to_string(),
            ],
        }
    }

    fn venus_cycle_fraction() -> f64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let synodic_venus_days = 583.92;
        let epoch = 1_600_000_000.0;
        ((now - epoch) / 86400.0 % synodic_venus_days) / synodic_venus_days
    }
}

impl BottomModule for ZodiacPanel {
    fn id(&self) -> ModuleId {
        ModuleId::Zodiac
    }

    fn draw_background(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        self.draw(canvas, x, y, w, h);
    }

    fn title(&self) -> (String, u32) {
        ("Zodiac".to_string(), 0xCC88FF)
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
        self.current = current_zodiac_sign();
        let f = Self::venus_cycle_fraction();
        self.lines[0] = format!("{} {}", self.current.symbol, self.current.name);
        self.lines[1] = format!("Venus cycle: {:.0}%", f * 100.0);
    }
}

impl Panel for ZodiacPanel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        canvas.set_draw_color(Color::RGB(17, 17, 17));
        let _ = canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
        canvas.set_draw_color(Color::RGB(0xCC, 0x88, 0xFF));
        let _ = canvas.fill_rect(Rect::new(x + 4, y + 4, (w - 8) as u32, 3));

        // Draw the current zodiac sign as a nice playful cartoony icon (hi/lo supported)
        let icon_size = ((h - 24).max(64) as u32).min(96);
        let icon_x = x + 12;
        let icon_y = y + (h - icon_size as i32) / 2;

        let icon_path = format!("zodiac/zodiac-{}-symbolic.svg", self.current.slug);
        crate::icons::draw_icon(
            canvas,
            &icon_path,
            icon_x,
            icon_y,
            icon_size,
        );

        let icon_size2 = ((h - 20).max(80) as u32).min(112);
        let icon_x2 = x + w - icon_size2 as i32 - 6;
        let icon_y2 = y + (h - icon_size2 as i32) / 2;
        crate::icons::draw_icon(
            canvas,
            "apps/calendar-symbolic.svg", // fallback decorative
            icon_x2,
            icon_y2,
            icon_size2,
        );
    }

    fn update(&mut self) {}
}
