use crate::drivers::platform::Platform;
use crate::drivers::sd_storage::{SdStorage, StorageBusMode};
use crate::platform::dvi_gfx::DviGfx;
use crate::prelude::*;
use crate::time_util::WallTime;
use crate::timing::advance_ms;

/// Seconds since boot, seeded to 07:00:00 for bring-up until DS3231 sets wall time.
static mut WALL_SECONDS: u32 = 7 * 3600;

pub struct PicoDviPlatform {
    sd: SdStorage,
    gfx: DviGfx,
    boot_frame: bool,
}

impl PicoDviPlatform {
    pub fn new() -> Self {
        Self {
            sd: SdStorage::new(StorageBusMode::I2c),
            gfx: DviGfx::new(),
            boot_frame: true,
        }
    }
}

fn busy_wait_ms(ms: u64) {
    // Approximate @ 252 MHz after DVI overclock.
    let cycles = ms.saturating_mul(252_000);
    cortex_m::asm::delay(cycles as u32);
}

impl Platform for PicoDviPlatform {
    async fn init(&mut self) -> Result<(), String> {
        self.sd.mount()
    }

    async fn draw_text(&mut self, _text: &str, _x: i32, _y: i32, _size: u8, _color: u32) {}

    async fn show_boot_splash(&mut self, status: &str) {
        self.gfx.present_splash_frame(status).await;
    }

    async fn finish_boot(&mut self) {
        self.boot_frame = false;
    }

    async fn draw_line(&mut self, _x1: i32, _y1: i32, _x2: i32, _y2: i32, _color: u32, _thickness: u8) {}

    async fn draw_circle(&mut self, _cx: i32, _cy: i32, _radius: i32, _color: u32) {}

    async fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        self.gfx.fill_rect(x, y, w, h, color).await;
    }

    async fn clear(&mut self) {
        self.gfx.clear(0x000000).await;
    }

    async fn clear_center_area(&mut self) {
        let layout = crate::layout::l();
        self.draw_rect(
            layout.center_x,
            layout.center_y,
            layout.center_w as i32,
            layout.center_h as i32,
            0x000000,
        )
        .await;
    }

    async fn present(&mut self) {
        if self.boot_frame {
            return;
        }
        let now = self.get_current_time();
        self.gfx
            .present_clock_frame(now.hour, now.minute, now.second)
            .await;
    }

    async fn play_sound(&mut self, _name: &str, _volume: f32) {}
    async fn play_raw_audio(&mut self, _path: &str) {}

    async fn fetch_weather(&self) -> Result<(i32, String), String> {
        Ok((68, String::from("Sunny")))
    }

    fn get_current_time(&self) -> WallTime {
        let total = unsafe { WALL_SECONDS };
        WallTime::new((total / 3600) % 24, (total / 60) % 60, total % 60)
    }

    fn delay_ms(&self, ms: u64) {
        let add = (ms / 1000).max(1) as u32;
        unsafe {
            WALL_SECONDS = WALL_SECONDS.saturating_add(add);
        }
        advance_ms(ms);
    }

    async fn delay(&self, ms: u64) {
        busy_wait_ms(ms);
        self.delay_ms(ms);
    }

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