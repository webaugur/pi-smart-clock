use sdl2::render::Canvas;
use sdl2::video::Window;

use super::module_id::ModuleId;

pub struct PanelLine {
    pub text: String,
    /// 0 = use layout `bottom_body_pt`
    pub size_pt: u8,
}

pub trait BottomModule {
    fn id(&self) -> ModuleId;
    fn draw_background(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32);
    fn title(&self) -> (String, u32);
    fn lines(&self) -> Vec<PanelLine>;
    fn tick(&mut self, _alerts_active: bool) {}
}