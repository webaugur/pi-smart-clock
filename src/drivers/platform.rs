#![allow(async_fn_in_trait)]

#[cfg(not(feature = "linux-full"))]
use crate::prelude::*;

#[cfg(feature = "linux-full")]
use chrono::{DateTime, Local};

#[cfg(not(feature = "linux-full"))]
use crate::time_util::WallTime;

/// Hardware abstraction for Pico (Embassy) and Linux (SDL2) builds.
pub trait Platform {
    async fn init(&mut self) -> Result<(), String>;

    // Display
    async fn draw_text(&mut self, text: &str, x: i32, y: i32, size: u8, color: u32);
    async fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32, thickness: u8);
    async fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, color: u32);
    async fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32);
    async fn draw_clock_face(&mut self, _cx: i32, _cy: i32, _diameter: u32) {}
    async fn draw_clock_second_hand(
        &mut self,
        _cx: i32,
        _cy: i32,
        _length: i32,
        _angle_deg: f32,
        _night: bool,
    ) {
    }
    async fn clear(&mut self);
    async fn clear_center_area(&mut self);
    async fn present(&mut self);

    // Audio
    async fn play_sound(&mut self, name: &str, volume: f32);
    async fn play_raw_audio(&mut self, path: &str);
    async fn play_alarm_loop(&mut self, _path: &str) {
        self.play_sound(_path, 0.9).await;
    }
    async fn stop_alarm_sound(&mut self) {}

    // Time
    #[cfg(feature = "linux-full")]
    fn get_current_time(&self) -> DateTime<Local>;
    #[cfg(not(feature = "linux-full"))]
    fn get_current_time(&self) -> WallTime;
    fn delay_ms(&self, ms: u64);
    async fn delay(&self, ms: u64);

    // Network / ESP8266 (no-op on Linux dev build)
    async fn fetch_weather(&self) -> Result<(i32, String), String>;
    async fn fetch_calendar(&self) -> Result<(), String> {
        Ok(())
    }
    async fn fetch_holidays(&self) -> Result<(), String> {
        Ok(())
    }
    async fn esp8266_mqtt_connect(
        &mut self,
        _broker: &str,
        _port: u16,
        _user: Option<&str>,
        _pass: Option<&str>,
    ) {
    }
    async fn esp8266_mqtt_publish(&mut self, _topic: &str, _payload: &str, _retain: bool) {}
    async fn esp8266_mqtt_subscribe(&mut self, _topic: &str) {}
    async fn esp8266_get_ntp(&mut self, _server: &str) -> Option<String> {
        None
    }
    async fn http_download_binary(&mut self, _url: &str) -> Option<Vec<u8>> {
        None
    }

    // Storage
    async fn write_file(&mut self, _path: &str, _data: &[u8]) {}
    async fn read_file(&mut self, _path: &str) -> Option<Vec<u8>> {
        None
    }
    async fn copy_file(&mut self, _from: &str, _to: &str) {}
    async fn save_photo_as_bmp(&mut self, _path: &str, _data: &[u8]) {}
    async fn create_official_placeholder(&mut self, _path: &str) {}

    // Input
    fn read_rotary_delta(&mut self) -> i32 {
        0
    }
    fn read_pushbutton(&mut self) -> bool {
        false
    }
    async fn read_i2s_samples(&mut self, _count: usize) -> Vec<i16> {
        Vec::new()
    }

    // Voice / UI helpers
    async fn speak(&mut self, _message: &str) {}
    async fn show_weather(&mut self) {}

    // OTA / flash (Pico only)
    async fn flash_backup_current(&mut self) {}
    async fn flash_write(&mut self, _offset: u32, _data: &[u8]) {}
    async fn set_boot_flag(&mut self, _flag: u32) {}
    fn reboot(&mut self) {}

    fn is_linux(&self) -> bool;
    fn is_tock(&self) -> bool;
    async fn run_forever(&mut self);
}