use chrono::{DateTime, Local};

use crate::drivers::platform::Platform;
use crate::drivers::sd_storage::{SdStorage, StorageBusMode};

pub struct PicoDviPlatform {
    sd: SdStorage,
}

impl PicoDviPlatform {
    pub fn new() -> Self {
        Self {
            sd: SdStorage::new(StorageBusMode::I2c),
        }
    }
}

impl Platform for PicoDviPlatform {
    async fn init(&mut self) -> Result<(), String> {
        self.sd.mount()
    }

    async fn draw_text(&mut self, _text: &str, _x: i32, _y: i32, _size: u8, _color: u32) {}
    async fn draw_line(&mut self, _x1: i32, _y1: i32, _x2: i32, _y2: i32, _color: u32, _thickness: u8) {}
    async fn draw_circle(&mut self, _cx: i32, _cy: i32, _radius: i32, _color: u32) {}
    async fn draw_rect(&mut self, _x: i32, _y: i32, _w: i32, _h: i32, _color: u32) {}
    async fn clear(&mut self) {}
    async fn clear_center_area(&mut self) {}
    async fn present(&mut self) {}
    async fn play_sound(&mut self, _name: &str, _volume: f32) {}
    async fn play_raw_audio(&mut self, _path: &str) {}

    async fn fetch_weather(&self) -> Result<(i32, String), String> {
        Ok((68, "Sunny".to_string()))
    }

    fn get_current_time(&self) -> DateTime<Local> {
        Local::now()
    }

    fn delay_ms(&self, _ms: u64) {}
    async fn delay(&self, _ms: u64) {}

    async fn write_file(&mut self, path: &str, data: &[u8]) {
        let resolved = crate::storage::embedded::resolve_logical_path(path);
        let _ = self.sd.write_file(&resolved, data);
    }

    async fn read_file(&mut self, path: &str) -> Option<Vec<u8>> {
        let resolved = crate::storage::embedded::resolve_logical_path(path);
        self.sd.read_file(&resolved).ok()
    }

    async fn copy_file(&mut self, from: &str, to: &str) {
        let from = crate::storage::embedded::resolve_logical_path(from);
        let to = crate::storage::embedded::resolve_logical_path(to);
        let _ = self.sd.copy_file(&from, &to);
    }

    fn is_linux(&self) -> bool {
        false
    }

    fn is_tock(&self) -> bool {
        false
    }

    async fn run_forever(&mut self) {
        loop {
            self.delay(1000).await;
        }
    }
}