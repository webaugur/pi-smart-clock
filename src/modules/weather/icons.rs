use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Internal coordinate space — scaled to the requested draw size.
const NATIVE: f32 = 128.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeatherIcon {
    Clear,
    PartlyCloudy,
    Cloudy,
    Fog,
    Drizzle,
    Rain,
    Snow,
    Thunderstorm,
    Unknown,
}

impl WeatherIcon {
    pub fn from_code(code: u16) -> Self {
        match code {
            0 => Self::Clear,
            1 | 2 => Self::PartlyCloudy,
            3 => Self::Cloudy,
            45 | 48 => Self::Fog,
            51 | 53 | 55 | 56 | 57 => Self::Drizzle,
            61 | 63 | 65 | 66 | 67 | 80 | 81 | 82 => Self::Rain,
            71 | 73 | 75 | 77 | 85 | 86 => Self::Snow,
            95 | 96 | 99 => Self::Thunderstorm,
            _ => Self::Unknown,
        }
    }
}

pub fn wmo_condition(code: u16) -> &'static str {
    match code {
        0 => "Clear sky",
        1 => "Mainly clear",
        2 => "Partly cloudy",
        3 => "Overcast",
        45 => "Fog",
        48 => "Depositing rime fog",
        51 => "Light drizzle",
        53 => "Drizzle",
        55 => "Dense drizzle",
        56 => "Freezing drizzle",
        57 => "Dense freezing drizzle",
        61 => "Slight rain",
        63 => "Rain",
        65 => "Heavy rain",
        66 => "Freezing rain",
        67 => "Heavy freezing rain",
        71 => "Slight snow",
        73 => "Snow",
        75 => "Heavy snow",
        77 => "Snow grains",
        80 => "Rain showers",
        81 => "Heavy rain showers",
        82 => "Violent rain showers",
        85 => "Snow showers",
        86 => "Heavy snow showers",
        95 => "Thunderstorm",
        96 => "Thunderstorm with hail",
        99 => "Thunderstorm with heavy hail",
        _ => "Unknown",
    }
}

pub fn draw_weather_icon(
    canvas: &mut Canvas<Window>,
    icon: WeatherIcon,
    x: i32,
    y: i32,
    size: u32,
) {
    let ctx = IconCtx::new(x, y, size);
    match icon {
        WeatherIcon::Clear => draw_clear(&ctx, canvas),
        WeatherIcon::PartlyCloudy => draw_partly_cloudy(&ctx, canvas),
        WeatherIcon::Cloudy => draw_cloudy(&ctx, canvas, false),
        WeatherIcon::Fog => draw_cloudy(&ctx, canvas, true),
        WeatherIcon::Drizzle => draw_precip(&ctx, canvas, Precip::Drizzle),
        WeatherIcon::Rain => draw_precip(&ctx, canvas, Precip::Rain),
        WeatherIcon::Snow => draw_snow(&ctx, canvas),
        WeatherIcon::Thunderstorm => draw_thunderstorm(&ctx, canvas),
        WeatherIcon::Unknown => draw_unknown(&ctx, canvas),
    }
}

struct IconCtx {
    x: i32,
    y: i32,
    scale: f32,
    cx: f32,
    cy: f32,
}

impl IconCtx {
    fn new(x: i32, y: i32, size: u32) -> Self {
        let scale = size as f32 / NATIVE;
        Self {
            x,
            y,
            scale,
            cx: NATIVE / 2.0,
            cy: NATIVE / 2.0,
        }
    }

    fn px(&self, nx: f32) -> i32 {
        self.x + (nx * self.scale).round() as i32
    }

    fn py(&self, ny: f32) -> i32 {
        self.y + (ny * self.scale).round() as i32
    }

    fn pr(&self, nr: f32) -> i32 {
        (nr * self.scale).round().max(1.0) as i32
    }
}

fn draw_clear(ctx: &IconCtx, canvas: &mut Canvas<Window>) {
    fill_circle(canvas, ctx.px(ctx.cx), ctx.py(ctx.cy), ctx.pr(30.0), Color::RGB(255, 200, 48));
    fill_circle(
        canvas,
        ctx.px(ctx.cx - 4.0),
        ctx.py(ctx.cy - 4.0),
        ctx.pr(24.0),
        Color::RGB(255, 224, 96),
    );
    canvas.set_draw_color(Color::RGB(255, 236, 140));
    for i in 0..8 {
        let ang = (i as f32 * 45.0).to_radians();
        let x1 = ctx.cx + ang.sin() * 38.0;
        let y1 = ctx.cy - ang.cos() * 38.0;
        let x2 = ctx.cx + ang.sin() * 50.0;
        let y2 = ctx.cy - ang.cos() * 50.0;
        thick_line(
            canvas,
            ctx.px(x1),
            ctx.py(y1),
            ctx.px(x2),
            ctx.py(y2),
            ctx.pr(3.0).max(2),
            Color::RGB(255, 220, 80),
        );
    }
}

fn draw_partly_cloudy(ctx: &IconCtx, canvas: &mut Canvas<Window>) {
    fill_circle(
        canvas,
        ctx.px(ctx.cx - 18.0),
        ctx.py(ctx.cy - 20.0),
        ctx.pr(22.0),
        Color::RGB(255, 200, 48),
    );
    fill_circle(
        canvas,
        ctx.px(ctx.cx - 20.0),
        ctx.py(ctx.cy - 22.0),
        ctx.pr(17.0),
        Color::RGB(255, 224, 96),
    );
    draw_cloud(
        ctx,
        canvas,
        ctx.cx + 8.0,
        ctx.cy + 10.0,
        72.0,
        Color::RGB(175, 188, 205),
        Color::RGB(145, 158, 178),
    );
}

fn draw_cloudy(ctx: &IconCtx, canvas: &mut Canvas<Window>, fog: bool) {
    let (top, bottom) = if fog {
        (Color::RGB(155, 165, 180), Color::RGB(125, 135, 150))
    } else {
        (Color::RGB(175, 188, 205), Color::RGB(145, 158, 178))
    };
    draw_cloud(ctx, canvas, ctx.cx, ctx.cy - 4.0, 88.0, top, bottom);
    if fog {
        canvas.set_draw_color(Color::RGB(190, 198, 210));
        for i in 0..4 {
            let ly = ctx.py(ctx.cy + 28.0 + i as f32 * 10.0);
            let thick = ctx.pr(2.0).max(1);
            let _ = canvas.fill_rect(Rect::new(
                ctx.px(18.0),
                ly,
                (ctx.pr(92.0) * 2) as u32,
                thick as u32,
            ));
        }
    }
}

enum Precip {
    Drizzle,
    Rain,
}

fn draw_precip(ctx: &IconCtx, canvas: &mut Canvas<Window>, kind: Precip) {
    draw_cloud(
        ctx,
        canvas,
        ctx.cx,
        ctx.cy - 18.0,
        76.0,
        Color::RGB(165, 178, 195),
        Color::RGB(135, 148, 168),
    );
    let drops = match kind {
        Precip::Drizzle => 5,
        Precip::Rain => 6,
    };
    let color = match kind {
        Precip::Drizzle => Color::RGB(110, 175, 255),
        Precip::Rain => Color::RGB(72, 145, 255),
    };
    canvas.set_draw_color(color);
    for i in 0..drops {
        let t = i as f32 / (drops - 1) as f32;
        let dx = ctx.cx - 28.0 + t * 56.0;
        let len = match kind {
            Precip::Drizzle => 14.0,
            Precip::Rain => 22.0,
        };
        thick_line(
            canvas,
            ctx.px(dx),
            ctx.py(ctx.cy + 18.0),
            ctx.px(dx - 6.0),
            ctx.py(ctx.cy + 18.0 + len),
            ctx.pr(2.5).max(2),
            color,
        );
    }
}

fn draw_snow(ctx: &IconCtx, canvas: &mut Canvas<Window>) {
    draw_cloud(
        ctx,
        canvas,
        ctx.cx,
        ctx.cy - 18.0,
        76.0,
        Color::RGB(165, 178, 195),
        Color::RGB(135, 148, 168),
    );
    let color = Color::RGB(220, 238, 255);
    for i in 0..5 {
        let t = i as f32 / 4.0;
        let sx = ctx.cx - 30.0 + t * 60.0;
        let sy = ctx.cy + 24.0 + (i % 2) as f32 * 8.0;
        draw_snowflake(canvas, ctx.px(sx), ctx.py(sy), ctx.pr(5.0).max(3), color);
    }
}

fn draw_snowflake(canvas: &mut Canvas<Window>, cx: i32, cy: i32, r: i32, color: Color) {
    canvas.set_draw_color(color);
    for i in 0..6 {
        let ang = (i as f32 * 60.0).to_radians();
        let x2 = cx + (ang.sin() * r as f32) as i32;
        let y2 = cy - (ang.cos() * r as f32) as i32;
        let _ = canvas.draw_line(sdl2::rect::Point::new(cx, cy), sdl2::rect::Point::new(x2, y2));
    }
    fill_circle(canvas, cx, cy, (r / 3).max(1), color);
}

fn draw_thunderstorm(ctx: &IconCtx, canvas: &mut Canvas<Window>) {
    draw_cloud(
        ctx,
        canvas,
        ctx.cx,
        ctx.cy - 20.0,
        80.0,
        Color::RGB(120, 132, 150),
        Color::RGB(90, 102, 120),
    );
    let bolt = [
        (ctx.cx + 4.0, ctx.cy + 10.0),
        (ctx.cx - 6.0, ctx.cy + 38.0),
        (ctx.cx + 8.0, ctx.cy + 38.0),
        (ctx.cx - 4.0, ctx.cy + 66.0),
        (ctx.cx + 14.0, ctx.cy + 34.0),
        (ctx.cx + 0.0, ctx.cy + 34.0),
    ];
    fill_polygon(canvas, ctx, &bolt, Color::RGB(255, 220, 48));
    fill_polygon(
        canvas,
        ctx,
        &[(ctx.cx + 2.0, ctx.cy + 14.0), (ctx.cx - 2.0, ctx.cy + 36.0), (ctx.cx + 6.0, ctx.cy + 36.0)],
        Color::RGB(255, 244, 160),
    );
}

fn draw_unknown(ctx: &IconCtx, canvas: &mut Canvas<Window>) {
    stroke_circle(
        canvas,
        ctx.px(ctx.cx),
        ctx.py(ctx.cy),
        ctx.pr(34.0),
        ctx.pr(3.0).max(2),
        Color::RGB(130, 140, 165),
    );
    canvas.set_draw_color(Color::RGB(160, 170, 190));
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(ctx.px(ctx.cx - 16.0), ctx.py(ctx.cy + 10.0)),
        sdl2::rect::Point::new(ctx.px(ctx.cx + 4.0), ctx.py(ctx.cy - 22.0)),
    );
    fill_circle(
        canvas,
        ctx.px(ctx.cx + 4.0),
        ctx.py(ctx.cy + 14.0),
        ctx.pr(4.0).max(2),
        Color::RGB(160, 170, 190),
    );
}

fn draw_cloud(
    ctx: &IconCtx,
    canvas: &mut Canvas<Window>,
    cx: f32,
    cy: f32,
    w: f32,
    top: Color,
    bottom: Color,
) {
    let bumps = [
        (cx - w * 0.28, cy + 2.0, w * 0.22),
        (cx - w * 0.08, cy - w * 0.12, w * 0.26),
        (cx + w * 0.18, cy - w * 0.04, w * 0.24),
        (cx + w * 0.34, cy + 4.0, w * 0.18),
    ];
    for (bx, by, r) in bumps {
        fill_circle(canvas, ctx.px(bx), ctx.py(by), ctx.pr(r), bottom);
    }
    for (bx, by, r) in bumps {
        fill_circle(
            canvas,
            ctx.px(bx),
            ctx.py(by - 2.0),
            ctx.pr(r * 0.88),
            top,
        );
    }
    let _ = canvas.fill_rect(Rect::new(
        ctx.px(cx - w * 0.42),
        ctx.py(cy + 2.0),
        ctx.pr(w * 0.84) as u32,
        ctx.pr(w * 0.22) as u32,
    ));
}

fn fill_polygon(canvas: &mut Canvas<Window>, ctx: &IconCtx, points: &[(f32, f32)], color: Color) {
    if points.len() < 3 {
        return;
    }
    canvas.set_draw_color(color);
    let min_y = points.iter().map(|p| p.1).fold(f32::INFINITY, f32::min);
    let max_y = points.iter().map(|p| p.1).fold(f32::NEG_INFINITY, f32::max);
    let mut y = min_y;
    while y <= max_y {
        let mut xs = Vec::new();
        for i in 0..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % points.len()];
            if (y1 <= y && y < y2) || (y2 <= y && y < y1) {
                let t = (y - y1) / (y2 - y1);
                xs.push(x1 + t * (x2 - x1));
            }
        }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut i = 0;
        while i + 1 < xs.len() {
            let x_start = ctx.px(xs[i]);
            let x_end = ctx.px(xs[i + 1]);
            let w = (x_end - x_start).max(1) as u32;
            let _ = canvas.fill_rect(Rect::new(x_start, ctx.py(y), w, ctx.pr(1.0).max(1) as u32));
            i += 2;
        }
        y += 1.0;
    }
}

fn fill_circle(canvas: &mut Canvas<Window>, cx: i32, cy: i32, radius: i32, color: Color) {
    if radius <= 0 {
        return;
    }
    canvas.set_draw_color(color);
    for dy in -radius..=radius {
        let dx = (((radius * radius - dy * dy) as f32).sqrt()) as i32;
        let w = (dx * 2).max(1) as u32;
        let _ = canvas.fill_rect(Rect::new(cx - dx, cy + dy, w, 1));
    }
}

fn stroke_circle(canvas: &mut Canvas<Window>, cx: i32, cy: i32, radius: i32, thick: i32, color: Color) {
    for r in (radius - thick).max(0)..=radius {
        fill_circle(canvas, cx, cy, r, color);
    }
}

fn thick_line(
    canvas: &mut Canvas<Window>,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    thick: i32,
    color: Color,
) {
    canvas.set_draw_color(color);
    let steps = ((x2 - x1).abs() + (y2 - y1).abs()).max(1);
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = x1 as f32 + t * (x2 - x1) as f32;
        let y = y1 as f32 + t * (y2 - y1) as f32;
        let _ = canvas.fill_rect(Rect::new(
            x.round() as i32 - thick / 2,
            y.round() as i32 - thick / 2,
            thick as u32,
            thick as u32,
        ));
    }
}