use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::layout::HandAsset;
use super::svg;

const RASTER_SIZE: u32 = super::RASTER_SIZE;

pub struct HandSprite {
    pub pixels: Vec<u8>,
    pub design_length: f32,
}

impl HandSprite {
    pub fn load(asset: HandAsset) -> Option<Self> {
        let path = svg::resolve_face_path(asset.file);
        let pixels = svg::rasterize_hand_from_path(&path, RASTER_SIZE)?;
        Some(Self {
            pixels,
            design_length: asset.design_length,
        })
    }
}

pub fn draw_hand_sprite(
    canvas: &mut Canvas<Window>,
    sprite: &HandSprite,
    cx: i32,
    cy: i32,
    screen_length: i32,
    angle_deg: f32,
    tint: Option<Color>,
) {
    if sprite.design_length <= 0.0 || screen_length <= 0 {
        return;
    }
    let dest = (RASTER_SIZE as f32 * screen_length as f32 / sprite.design_length).round() as u32;
    let x = cx - (dest as i32 / 2);
    let y = cy - (dest as i32 / 2);
    svg::draw_rgba_ex(
        canvas,
        &sprite.pixels,
        RASTER_SIZE,
        RASTER_SIZE,
        x,
        y,
        dest,
        dest,
        angle_deg as f64,
        tint,
    );
}

pub fn ivory_tint(night: bool) -> Option<Color> {
    if night {
        Some(Color::RGB(220, 200, 170))
    } else {
        None
    }
}

pub fn second_tint(night: bool) -> Option<Color> {
    if night {
        Some(Color::RGB(255, 120, 64))
    } else {
        None
    }
}