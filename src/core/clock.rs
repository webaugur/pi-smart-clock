use crate::drivers::platform::Platform;
use chrono::{Local, Timelike};

pub async fn update<P: Platform>(platform: &mut P) {
    let now = Local::now();
    let seconds = now.second() as f32;
    let bounce = (seconds.fract() * 8.0).sin().abs() * 3.0;
    let angle = (seconds * 6.0) + bounce;

    let is_night = now.hour() >= 22 || now.hour() < 6;
    let bg = if is_night { 0x0A0A0A } else { 0x000000 };
    platform.draw_rect(0, 0, 800, 480, bg).await;

    platform.draw_circle(400, 200, 190, 0xFFFFFF).await;
    platform
        .draw_circle(400, 200, 175, if is_night { 0x1A1400 } else { 0x111111 })
        .await;

    let amber = if is_night { 0xFFAA33 } else { 0xFFFFFF };
    for i in 0..12 {
        let ang = (i as f32 * 30.0).to_radians();
        let x1 = (400.0 + ang.sin() * 190.0) as i32;
        let y1 = (200.0 - ang.cos() * 190.0) as i32;
        let x2 = (400.0 + ang.sin() * 170.0) as i32;
        let y2 = (200.0 - ang.cos() * 170.0) as i32;
        platform.draw_line(x1, y1, x2, y2, amber, 2).await;
    }

    let rad = angle.to_radians();
    let hx = (400.0 + rad.sin() * 165.0) as i32;
    let hy = (200.0 - rad.cos() * 165.0) as i32;
    platform.draw_line(400, 200, hx, hy, 0xFF2222, 4).await;
}
