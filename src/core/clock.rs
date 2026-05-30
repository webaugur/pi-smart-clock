use crate::drivers::platform::Platform;
use chrono::Local;
use crate::config;

pub async fn update<P: Platform>(platform: &mut P) {
    let now = Local::now();
    let seconds = now.second() as f32;
    let bounce = (seconds.fract() * 8.0).sin().abs() * 3.0;
    let angle = (seconds * 6.0) + bounce;

    let is_night = now.hour() >= 22 || now.hour() < 6;

    // Background
    let bg = if is_night { 0x0A0A0A } else { 0x000000 };
    platform.draw_rect(0, 0, 800, 480, bg);

    // Clock face
    platform.draw_circle(400, 200, 190, 0xFFFFFF);
    platform.draw_circle(400, 200, 175, if is_night { 0x1A1400 } else { 0x111111 });

    // Roman numerals with candle flicker at night
    const ROMAN: [&str; 12] = ["XII", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI"];
    let brightness = if is_night {
        let t = (now.timestamp() as f32 * 0.8).sin() * 0.08 + 0.92;
        let noise = (rand::random::<f32>() - 0.5) * 0.04;
        (t + noise).clamp(0.85, 1.0)
    } else { 1.0 };

    let amber = if is_night { 0xFFAA33 } else { 0xFFFFFF };

    for i in 0..12 {
        let ang = (i as f32 * 30.0).to_radians();
        let x = (400.0 + ang.sin() * 155.0) as i32;
        let y = (200.0 - ang.cos() * 155.0) as i32;
        platform.draw_text(ROMAN[i], x - 12, y - 10, 18, amber);
    }

    // Red second hand
    let rad = angle.to_radians();
    let hx = (400.0 + rad.sin() * 165.0) as i32;
    let hy = (200.0 - rad.cos() * 165.0) as i32;
    platform.draw_line(400, 200, hx, hy, 0xFF2222, 4);
}