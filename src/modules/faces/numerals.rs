use chrono::{Local, Timelike};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::layout::FaceLayout;
use super::svg::{self, RasterGlyph};

const RASTER_SIZE: u32 = 512;

pub fn draw_positioned(
    canvas: &mut Canvas<Window>,
    glyphs: &[RasterGlyph; 12],
    face: FaceLayout,
    cx: i32,
    cy: i32,
    diameter: u32,
) {
    let is_night = Local::now().hour() >= 22 || Local::now().hour() < 6;
    let tint = if is_night {
        Some(Color::RGB(255, 170, 51))
    } else {
        None
    };
    let scale = face.screen_scale(diameter, RASTER_SIZE);

    for i in 0..12 {
        let glyph = &glyphs[i];
        if glyph.width == 0 || glyph.height == 0 {
            continue;
        }
        let (svg_x, svg_y, angle) = face.numeral_anchor(i as u32);
        let dest_w = ((glyph.width as f32) * scale).round().max(1.0) as u32;
        let dest_h = ((glyph.height as f32) * scale).round().max(1.0) as u32;
        let px = (cx as f32 + (svg_x - face.center) * scale).round() as i32;
        let py = (cy as f32 + (svg_y - face.center) * scale).round() as i32;
        let x = px - dest_w as i32 / 2;
        let y = py - dest_h as i32 / 2;
        svg::draw_rgba_ex(
            canvas,
            &glyph.pixels,
            glyph.width,
            glyph.height,
            x,
            y,
            dest_w,
            dest_h,
            angle as f64,
            tint,
        );
    }
}