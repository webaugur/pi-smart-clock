use chrono::{DateTime, Local, Timelike};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

use crate::layout::{
    BOTTOM_H, BOTTOM_Y, CENTER_H, CENTER_W, CENTER_X, CENTER_Y, CLOCK_CX, CLOCK_CY, ROMAN_RADIUS,
    SCREEN_W,
};

const ROMAN: [&str; 12] = [
    "XII", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI",
];

pub fn draw_layout_regions(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(25, 25, 35));
    canvas
        .fill_rect(Rect::new(0, BOTTOM_Y, SCREEN_W as u32, BOTTOM_H as u32))
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(15, 15, 25));
    canvas
        .fill_rect(Rect::new(
            CENTER_X,
            CENTER_Y,
            CENTER_W,
            CENTER_H,
        ))
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn draw_roman_numerals(
    canvas: &mut Canvas<Window>,
    font: &Font,
    now: DateTime<Local>,
) -> Result<(), String> {
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
        let r = ROMAN_RADIUS as f32;
        let tx = (CLOCK_CX as f32 + ang.sin() * r) as i32 - q.width as i32 / 2;
        let ty = (CLOCK_CY as f32 - ang.cos() * r) as i32 - q.height as i32 / 2;
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