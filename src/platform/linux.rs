use crate::drivers::platform::Platform;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct LinuxPlatform {
    pub canvas: Option<Canvas<Window>>,
}

impl LinuxPlatform {
    pub fn new() -> Self { Self { canvas: None } }
}

impl Platform for LinuxPlatform {
    async fn init(&mut self) -> Result<(), String> { Ok(()) }
    async fn draw_text(&mut self, text: &str, x: i32, y: i32, size: u8, color: u32) {}
    async fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32, thickness: u8) {}
    async fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, color: u32) {}
    async fn play_sound(&mut self, name: &str, volume: f32) {}
    async fn fetch_weather(&self) -> Result<(i32, String), String> { Ok((72, "Partly Cloudy".to_string())) }
    fn delay_ms(&self, ms: u64) { std::thread::sleep(std::time::Duration::from_millis(ms)); }
    async fn delay(&self, ms: u64) { tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await; }
    async fn run_forever(&mut self) { loop {} }
    fn is_linux(&self) -> bool { true }
    fn is_tock(&self) -> bool { false }
}