use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::layout::l;

pub fn draw_layout_regions(canvas: &mut Canvas<Window>) -> Result<(), String> {
    let layout = l();
    canvas.set_draw_color(Color::RGB(17, 17, 17));
    canvas
        .fill_rect(Rect::new(
            0,
            layout.bottom_y,
            layout.screen_w as u32,
            layout.bottom_h as u32,
        ))
        .map_err(|e| e.to_string())?;
    Ok(())
}