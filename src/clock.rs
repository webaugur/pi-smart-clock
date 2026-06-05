use chrono::{DateTime, Local, Timelike};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

use crate::layout::l;

const ROMAN: [&str; 12] = [
    "XII", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI",
];

pub fn draw_layout_regions(canvas: &mut Canvas<Window>) -> Result<(), String> {
    let layout = l();
    canvas.set_draw_color(Color::RGB(25, 25, 35));
    canvas
        .fill_rect(Rect::new(
            0,
            layout.bottom_y,
            layout.screen_w as u32,
            layout.bottom_h as u32,
        ))
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(15, 15, 25));
    canvas
        .fill_rect(Rect::new(
            layout.center_x,
            layout.center_y,
            layout.center_w,
            layout.center_h,
        ))
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn draw_roman_numerals(
    canvas: &mut Canvas<Window>,
    font: &Font,
    now: DateTime<Local>,
) -> Result<(), String> {
    let layout = l();
    let is_night = now.hour() >= 22 || now.hour() < 6;
    let color = if is_night {
        Color::RGB(255, 170, 51)
    } else {
        Color::RGB(255, 255, 255)
    };

    let creator = canvas.texture_creator();
    for i in 0..12 {
        let ang = (i as f32 * 30.0).to_radians();
        let surface = font
            .render(ROMAN[i])
            .blended(color)
            .map_err(|e| e.to_string())?;
        let texture = creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let q = texture.query();
        let r = layout.roman_radius as f32;
        let tx = (layout.clock_cx as f32 + ang.sin() * r) as i32 - q.width as i32 / 2;
        let ty = (layout.clock_cy as f32 - ang.cos() * r) as i32 - q.height as i32 / 2;
        canvas
            .copy(
                &texture,
                None,
                Rect::new(tx, ty, q.width, q.height),
            )
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}