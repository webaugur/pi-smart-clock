use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::path::{Path, PathBuf};

const FONT_PATHS: &[&str] = &[
    "assets/fonts/DejaVuSerif-Bold.ttf",
    "/usr/share/fonts/truetype/dejavu/DejaVuSerif-Bold.ttf",
    "/usr/share/fonts/TTF/DejaVuSerif-Bold.ttf",
];

pub struct RasterGlyph {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub fn resolve_face_path(rel: &str) -> PathBuf {
    let bundled = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets/faces")
        .join(rel);
    if bundled.is_file() {
        return bundled;
    }
    PathBuf::from(rel)
}

pub fn parse_face_svg(path: &Path) -> Option<usvg::Tree> {
    let data = std::fs::read_to_string(path).ok()?;
    parse_face_svg_data(&data)
}

pub fn parse_face_svg_data(data: &str) -> Option<usvg::Tree> {
    let mut opt = usvg::Options::default();
    opt.font_family = "DejaVu Serif".to_string();
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for rel in FONT_PATHS {
        let path = if rel.starts_with('/') {
            PathBuf::from(rel)
        } else {
            manifest.join(rel)
        };
        if path.is_file() {
            let _ = opt.fontdb_mut().load_font_file(&path);
        }
    }
    usvg::Tree::from_str(data, &opt).ok()
}

pub fn rasterize_tree_node(tree: &usvg::Tree, id: &str, pad: u32) -> Option<RasterGlyph> {
    let node = tree.node_by_id(id)?;
    let bbox = node.abs_layer_bounding_box()?;
    if bbox.width() <= 0.0 || bbox.height() <= 0.0 {
        return None;
    }
    let w = bbox.width().ceil() as u32 + pad * 2;
    let h = bbox.height().ceil() as u32 + pad * 2;
    let mut pixmap = tiny_skia::Pixmap::new(w, h)?;
    resvg::render_node(node, tiny_skia::Transform::identity(), &mut pixmap.as_mut())?;
    Some(RasterGlyph {
        pixels: pixmap.data().to_vec(),
        width: w,
        height: h,
    })
}

pub fn rasterize_dial(tree: &usvg::Tree, size: u32) -> Option<Vec<u8>> {
    let node = tree.node_by_id("dial")?;
    let tree_size = tree.size();
    let scale = size as f32 / tree_size.width().max(tree_size.height());
    let w = (tree_size.width() * scale).ceil() as u32;
    let h = (tree_size.height() * scale).ceil() as u32;
    let mut pixmap = tiny_skia::Pixmap::new(w, h)?;
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    // Render the dial group in viewBox coordinates. `#dial` includes a hidden
    // 512×512 bounds rect so the bbox origin matches the face center.
    resvg::render_node(node, transform, &mut pixmap.as_mut())?;
    let mut out = vec![0u8; (size * size * 4) as usize];
    let x_off = ((size.saturating_sub(w)) / 2) as usize;
    let y_off = ((size.saturating_sub(h)) / 2) as usize;
    for row in 0..h as usize {
        let src = &pixmap.data()[row * w as usize * 4..(row + 1) * w as usize * 4];
        let dst_row = y_off + row;
        if dst_row >= size as usize {
            continue;
        }
        let dst_start = (dst_row * size as usize + x_off) * 4;
        let dst_end = dst_start + src.len();
        if dst_end <= out.len() {
            out[dst_start..dst_end].copy_from_slice(src);
        }
    }
    Some(out)
}

pub fn draw_rgba(
    canvas: &mut Canvas<Window>,
    pixels: &[u8],
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    dest_w: u32,
    dest_h: u32,
) {
    draw_rgba_ex(canvas, pixels, width, height, x, y, dest_w, dest_h, 0.0, None);
}

pub fn draw_rgba_ex(
    canvas: &mut Canvas<Window>,
    pixels: &[u8],
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    dest_w: u32,
    dest_h: u32,
    angle: f64,
    tint: Option<sdl2::pixels::Color>,
) {
    let mut buf = if let Some(color) = tint {
        tint_pixels(pixels, color)
    } else {
        pixels.to_vec()
    };
    let surface = match Surface::from_data(
        &mut buf,
        width,
        height,
        width * 4,
        PixelFormatEnum::RGBA32,
    ) {
        Ok(s) => s,
        Err(_) => return,
    };
    let creator = canvas.texture_creator();
    let mut texture = match creator.create_texture_from_surface(&surface) {
        Ok(t) => t,
        Err(_) => return,
    };
    let _ = texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    let dst = Rect::new(x, y, dest_w, dest_h);
    if angle.abs() < 0.01 {
        let _ = canvas.copy(&texture, None, dst);
    } else {
        let center = sdl2::rect::Point::new(
            (dest_w / 2) as i32,
            (dest_h / 2) as i32,
        );
        let _ = canvas.copy_ex(&texture, None, dst, angle, center, false, false);
    }
}

fn tint_pixels(base: &[u8], color: sdl2::pixels::Color) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retro_roman_dial_and_numerals_extract() {
        let path = resolve_face_path("retro-roman/face.svg");
        let tree = parse_face_svg(&path).expect("parse face");
        let dial = rasterize_dial(&tree, 512).expect("dial");
        let bezel = count_bright_pixels(&dial, 512, 240, 272, 8, 40);
        assert!(bezel > 20, "expected visible dial bezel, got {bezel}");

        let xii = rasterize_tree_node(&tree, "numeral-0", 2).expect("numeral-0");
        let bright = count_bright_pixels_sized(&xii.pixels, xii.width, 0, xii.width, 0, xii.height);
        assert!(bright > 40, "expected visible XII glyph, got {bright}");
    }

    fn count_bright_pixels(
        pixels: &[u8],
        size: u32,
        x0: u32,
        x1: u32,
        y0: u32,
        y1: u32,
    ) -> u32 {
        count_bright_pixels_sized(pixels, size, x0, x1, y0, y1)
    }

    fn count_bright_pixels_sized(
        pixels: &[u8],
        width: u32,
        x0: u32,
        x1: u32,
        y0: u32,
        y1: u32,
    ) -> u32 {
        let mut n = 0;
        for y in y0..y1.min(pixels.len() as u32 / (width * 4)) {
            for x in x0..x1.min(width) {
                let i = ((y * width + x) * 4) as usize;
                if pixels[i + 3] > 0 && pixels[i] > 200 {
                    n += 1;
                }
            }
        }
        n
    }
}