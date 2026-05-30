use crate::drivers::platform::Platform;
use chrono::Local;

pub async fn update<P: Platform>(platform: &mut P) {
    let now = Local::now();
    let seconds = now.second() as f32;
    let bounce = (seconds.fract() * 8.0).sin().abs() * 3.0;
    let angle = (seconds * 6.0) + bounce;

    // Draw clock face
    platform.draw_circle(400, 200, 190, 0xFFFFFF);
    platform.draw_circle(400, 200, 175, 0x111111);

    // Roman numerals
    const ROMAN: [&str; 12] = ["XII", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI"];
    for i in 0..12 {
        let ang = (i as f32 * 30.0).to_radians();
        let x = (400.0 + ang.sin() * 155.0) as i32;
        let y = (200.0 - ang.cos() * 155.0) as i32;
        platform.draw_text(ROMAN[i], x - 12, y - 10, 18, 0xFFAA33);
    }

    // Red second hand with bounce
    let rad = angle.to_radians();
    let hx = (400.0 + rad.sin() * 165.0) as i32;
    let hy = (200.0 - rad.cos() * 165.0) as i32;
    platform.draw_line(400, 200, hx, hy, 0xFF2222, 4);
}