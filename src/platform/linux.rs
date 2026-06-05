use chrono::{DateTime, Local};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use std::collections::HashMap;
use std::path::Path;

use crate::drivers::platform::Platform;

pub trait SdlPlatformExt {
    fn canvas_mut(&mut self) -> &mut Canvas<Window>;
    fn ingest_key(&mut self, key: Keycode, pressed: bool);
    fn set_font(&mut self, font: &'static Font<'static, 'static>);
    fn font(&self) -> Option<&'static Font<'static, 'static>>;
}

pub struct SdlPlatform {
    canvas: Canvas<Window>,
    font: Option<&'static Font<'static, 'static>>,
    rotary_delta: i32,
    button_down: bool,
    files: HashMap<String, Vec<u8>>,
}

impl SdlPlatform {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self {
            canvas,
            font: None,
            rotary_delta: 0,
            button_down: false,
            files: HashMap::new(),
        }
    }

    fn rgb(c: u32) -> Color {
        Color::RGB(((c >> 16) & 0xFF) as u8, ((c >> 8) & 0xFF) as u8, (c & 0xFF) as u8)
    }

    fn draw_circle_impl(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        self.canvas.set_draw_color(color);
        for r in (0..=radius).rev() {
            let d = (r * 2) as u32;
            let _ = self.canvas.fill_rect(Rect::new(cx - r, cy - r, d, d));
        }
    }
}

impl SdlPlatformExt for SdlPlatform {
    fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    fn set_font(&mut self, font: &'static Font<'static, 'static>) {
        self.font = Some(font);
    }

    fn font(&self) -> Option<&'static Font<'static, 'static>> {
        self.font
    }

    fn ingest_key(&mut self, key: Keycode, pressed: bool) {
        if !pressed {
            return;
        }
        match key {
            Keycode::Left | Keycode::Down => self.rotary_delta -= 1,
            Keycode::Right | Keycode::Up => self.rotary_delta += 1,
            Keycode::Space | Keycode::Return => self.button_down = true,
            _ => {}
        }
    }
}

impl Platform for SdlPlatform {
    async fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn draw_text(&mut self, text: &str, x: i32, y: i32, _size: u8, color: u32) {
        let Some(font) = self.font else {
            return;
        };
        let surface = match font.render(text).blended(Self::rgb(color)) {
            Ok(s) => s,
            Err(_) => return,
        };
        let creator = self.canvas.texture_creator();
        let texture = match creator.create_texture_from_surface(&surface) {
            Ok(t) => t,
            Err(_) => return,
        };
        let q = texture.query();
        let _ = self.canvas.copy(
            &texture,
            None,
            Rect::new(x, y, q.width, q.height),
        );
    }

    async fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32, _thickness: u8) {
        self.canvas.set_draw_color(Self::rgb(color));
        let _ = self.canvas.draw_line(
            sdl2::rect::Point::new(x1, y1),
            sdl2::rect::Point::new(x2, y2),
        );
    }

    async fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, color: u32) {
        self.draw_circle_impl(cx, cy, radius, Self::rgb(color));
    }

    async fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        self.canvas.set_draw_color(Self::rgb(color));
        let _ = self
            .canvas
            .fill_rect(Rect::new(x, y, w.max(1) as u32, h.max(1) as u32));
    }

    async fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        let _ = self.canvas.clear();
    }

    async fn clear_center_area(&mut self) {
        self.draw_rect(200, 120, 400, 280, 0x000000).await;
    }

    async fn present(&mut self) {
        let _ = self.canvas.present();
    }

    async fn play_sound(&mut self, path: &str, _volume: f32) {
        if Path::new(path).exists() {
            if let Ok(m) = sdl2::mixer::Music::from_file(path) {
                let _ = m.play(0);
            }
        }
    }

    async fn play_raw_audio(&mut self, path: &str) {
        self.play_sound(path, 1.0).await;
    }

    fn get_current_time(&self) -> DateTime<Local> {
        Local::now()
    }

    fn delay_ms(&self, ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }

    async fn delay(&self, ms: u64) {
        self.delay_ms(ms);
    }

    async fn fetch_weather(&self) -> Result<(i32, String), String> {
        Ok((72, "Partly Cloudy".to_string()))
    }

    async fn write_file(&mut self, path: &str, data: &[u8]) {
        self.files.insert(path.to_string(), data.to_vec());
    }

    async fn read_file(&mut self, path: &str) -> Option<Vec<u8>> {
        self.files
            .get(path)
            .cloned()
            .or_else(|| std::fs::read(path).ok())
    }

    async fn copy_file(&mut self, from: &str, to: &str) {
        if let Some(data) = self.read_file(from).await {
            self.write_file(to, &data).await;
        }
    }

    fn read_rotary_delta(&mut self) -> i32 {
        let d = self.rotary_delta;
        self.rotary_delta = 0;
        d
    }

    fn read_pushbutton(&mut self) -> bool {
        let b = self.button_down;
        self.button_down = false;
        b
    }

    async fn speak(&mut self, message: &str) {
        crate::core::voice_feedback::VoiceFeedback::speak(self, message).await;
    }

    async fn show_weather(&mut self) {}

    fn is_linux(&self) -> bool {
        true
    }

    fn is_tock(&self) -> bool {
        false
    }

    async fn run_forever(&mut self) {
        loop {
            self.delay(1000).await;
        }
    }

    fn reboot(&mut self) {}
}
