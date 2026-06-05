use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const RASTER_SIZE: u32 = 128;

static ATLAS: OnceLock<IconAtlas> = OnceLock::new();

pub fn draw_symbolic_icon(
    canvas: &mut Canvas<Window>,
    rel_path: &str,
    x: i32,
    y: i32,
    size: u32,
    tint: Color,
) {
    let atlas = ATLAS.get_or_init(IconAtlas::load);
    atlas.draw(canvas, rel_path, x, y, size, tint);
}

struct IconAtlas {
    icons: HashMap<String, Vec<u8>>,
}

impl IconAtlas {
    fn load() -> Self {
        let mut icons = HashMap::new();
        for rel in ICON_PATHS {
            if let Some(pixels) = rasterize_svg(&resolve_icon_path(rel)) {
                icons.insert(rel.to_string(), pixels);
            } else {
                eprintln!("[icons] failed to load {rel}");
            }
        }
        Self { icons }
    }

    fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        rel_path: &str,
        x: i32,
        y: i32,
        size: u32,
        tint: Color,
    ) {
        let Some(base) = self.icons.get(rel_path) else {
            return;
        };
        let mut tinted = tint_pixels(base, tint);
        let surface = match Surface::from_data(
            &mut tinted,
            RASTER_SIZE,
            RASTER_SIZE,
            RASTER_SIZE * 4,
            PixelFormatEnum::RGBA32,
        )
            {
                Ok(s) => s,
                Err(_) => return,
            };
        let creator = canvas.texture_creator();
        let mut texture = match creator.create_texture_from_surface(&surface) {
            Ok(t) => t,
            Err(_) => return,
        };
        let _ = texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        let _ = canvas.copy(&texture, None, Rect::new(x, y, size, size));
    }
}

const ICON_PATHS: &[&str] = &[
    "status/weather-clear-symbolic.svg",
    "status/weather-clear-night-symbolic.svg",
    "status/weather-few-clouds-symbolic.svg",
    "status/weather-overcast-symbolic.svg",
    "status/weather-fog-symbolic.svg",
    "status/weather-showers-scattered-symbolic.svg",
    "status/weather-showers-symbolic.svg",
    "status/weather-snow-symbolic.svg",
    "status/weather-storm-symbolic.svg",
    "status/adw-tab-icon-missing-symbolic.svg",
    "status/starred-symbolic.svg",
    "apps/calendar-symbolic.svg",
];

fn resolve_icon_path(rel: &str) -> PathBuf {
    let bundled = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/icons/yaru").join(rel);
    if bundled.is_file() {
        return bundled;
    }
    let mut system = PathBuf::from("/usr/share/icons/Yaru/scalable");
    system.push(rel);
    system
}

fn rasterize_svg(path: &Path) -> Option<Vec<u8>> {
    let data = std::fs::read_to_string(path).ok()?;
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(&data, &opt).ok()?;
    let size = tree.size();
    let scale = (RASTER_SIZE as f32 / size.width().max(size.height())).min(1.0);
    let w = (size.width() * scale).ceil() as u32;
    let h = (size.height() * scale).ceil() as u32;
    let mut pixmap = tiny_skia::Pixmap::new(w, h)?;
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    let mut out = vec![0u8; (RASTER_SIZE * RASTER_SIZE * 4) as usize];
    let x_off = ((RASTER_SIZE - w) / 2) as usize;
    let y_off = ((RASTER_SIZE - h) / 2) as usize;
    for row in 0..h as usize {
        let src = &pixmap.data()[row * w as usize * 4..(row + 1) * w as usize * 4];
        let dst_row = y_off + row;
        if dst_row >= RASTER_SIZE as usize {
            continue;
        }
        let dst_start = (dst_row * RASTER_SIZE as usize + x_off) * 4;
        let dst_end = dst_start + src.len();
        if dst_end <= out.len() {
            out[dst_start..dst_end].copy_from_slice(src);
        }
    }
    Some(out)
}

fn tint_pixels(base: &[u8], color: Color) -> Vec<u8> {
    let mut out = base.to_vec();
    for px in out.chunks_exact_mut(4) {
        let a = px[3];
        if a == 0 {
            continue;
        }
        let lum = ((px[0] as u16 + px[1] as u16 + px[2] as u16) / 3) as f32 / 255.0;
        px[0] = (color.r as f32 * lum) as u8;
        px[1] = (color.g as f32 * lum) as u8;
        px[2] = (color.b as f32 * lum) as u8;
    }
    out
}