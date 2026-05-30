use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::Window;
use chrono::Local;

const ROMAN: [&str; 12] = ["XII", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI"];

pub struct Clock {
    font: Font<'static>,
    texture_creator: TextureCreator<sdl2::video::WindowContext>,
}

impl Clock {
    pub fn new(font: &mut Font, texture_creator: &TextureCreator<sdl2::video::WindowContext>) -> Result<Self, String> {
        Ok(Self {
            font: font.to_owned(),
            texture_creator: texture_creator.to_owned(),
        })
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let now = Local::now();
        let seconds = now.second() as f32;
        let bounce = (seconds.fract() * 8.0).sin().abs() * 3.0;
        let angle = (seconds * 6.0) + bounce;

        let cx = 400;
        let cy = 240;
        let radius = 200;

        canvas.set_draw_color(Color::RGB(240, 240, 240));
        for i in 0..12 {
            let ang = (i as f32 * 30.0).to_radians();
            let x1 = cx + (ang.sin() * radius as f32) as i32;
            let y1 = cy - (ang.cos() * radius as f32) as i32;
            let x2 = cx + (ang.sin() * (radius - 20) as f32) as i32;
            let y2 = cy - (ang.cos() * (radius - 20) as f32) as i32;
            let _ = canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2));

            let text = ROMAN[i];
            let surface = self.font.render(text)
                .blended(Color::RGB(255, 255, 255))
                .map_err(|e| e.to_string())?;
            let texture = self.texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
            let query = texture.query();
            let dest = sdl2::rect::Rect::new(
                cx - (query.width as i32) / 2,
                (cy - (query.height as i32) / 2) - 25,
                query.width,
                query.height
            );
            let _ = canvas.copy(&texture, None, dest);
        }

        let rad = angle.to_radians();
        let hand_x = cx + (rad.sin() * 185.0) as i32;
        let hand_y = cy - (rad.cos() * 185.0) as i32;
        canvas.set_draw_color(Color::RGB(200, 20, 20));
        let _ = canvas.draw_line(Point::new(cx, cy), Point::new(hand_x, hand_y));

        Ok(())
    }
}