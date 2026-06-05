use crate::drivers::platform::Platform;
use crate::layout::{SCREEN_W, STATUS_Y};

pub async fn draw<P: Platform>(platform: &mut P, temp: f32, git_hash: &str) {
    let y = STATUS_Y;
    platform.draw_rect(0, y, SCREEN_W, 5, 0x112233).await;
    let speed_mhz = crate::config::CLOCK_SPEED_HZ / 1_000_000;
    let text = format!("{}MHz • {:.1}°C • {}", speed_mhz, temp, git_hash);
    platform.draw_text(&text, 8, y - 18, 14, 0x88AAFF).await;
}