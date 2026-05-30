use crate::drivers::platform::Platform;

pub async fn draw<P: Platform>(platform: &mut P, temp: f32, git_hash: &str) {
    let y = 475;
    platform.draw_rect(0, y, 800, 5, 0x112233);
    let speed_mhz = crate::config::CLOCK_SPEED_HZ / 1_000_000;
    let text = format!("{}MHz • {:.1}°C • {}", speed_mhz, temp, git_hash);
    platform.draw_text(&text, 8, y - 18, 14, 0x88AAFF);
}