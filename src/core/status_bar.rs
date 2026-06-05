use crate::drivers::platform::Platform;
use crate::layout::l;

pub async fn draw<P: Platform>(platform: &mut P, temp: f32, git_hash: &str) {
    let layout = l();
    let y = layout.status_y;
    platform.draw_rect(0, y, layout.screen_w, 5, 0x112233).await;
    let speed_mhz = crate::config::CLOCK_SPEED_HZ / 1_000_000;
    let text = format!("{}MHz • {:.1}°C • {}", speed_mhz, temp, git_hash);
    platform.draw_text(&text, 12, y - 28, 22, 0x88AAFF).await;
}