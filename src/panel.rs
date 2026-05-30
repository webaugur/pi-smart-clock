use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Panel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32);
    fn update(&mut self);
}