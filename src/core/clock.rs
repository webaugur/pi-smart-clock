use crate::drivers::platform::Platform;
use crate::layout::{
    CLOCK_CX, CLOCK_CY, CLOCK_INNER_R, CLOCK_OUTER_R, HAND_LENGTH, SCREEN_H, SCREEN_W,
    TICK_INNER_R, TICK_OUTER_R,
};
use chrono::{Local, Timelike};

pub async fn update<P: Platform>(platform: &mut P) {
    let now = Local::now();
    let seconds = now.second() as f32;
    let bounce = (seconds.fract() * 8.0).sin().abs() * 3.0;
    let angle = (seconds * 6.0) + bounce;

    let is_night = now.hour() >= 22 || now.hour() < 6;
    let bg = if is_night { 0x0A0A0A } else { 0x000000 };
    platform.draw_rect(0, 0, SCREEN_W, SCREEN_H, bg).await;

    platform
        .draw_circle(CLOCK_CX, CLOCK_CY, CLOCK_OUTER_R, 0xFFFFFF)
        .await;
    platform
        .draw_circle(
            CLOCK_CX,
            CLOCK_CY,
            CLOCK_INNER_R,
            if is_night { 0x1A1400 } else { 0x111111 },
        )
        .await;

    let amber = if is_night { 0xFFAA33 } else { 0xFFFFFF };
    for i in 0..12 {
        let ang = (i as f32 * 30.0).to_radians();
        let x1 = (CLOCK_CX as f32 + ang.sin() * TICK_OUTER_R as f32) as i32;
        let y1 = (CLOCK_CY as f32 - ang.cos() * TICK_OUTER_R as f32) as i32;
        let x2 = (CLOCK_CX as f32 + ang.sin() * TICK_INNER_R as f32) as i32;
        let y2 = (CLOCK_CY as f32 - ang.cos() * TICK_INNER_R as f32) as i32;
        platform.draw_line(x1, y1, x2, y2, amber, 2).await;
    }

    let rad = angle.to_radians();
    let hx = (CLOCK_CX as f32 + rad.sin() * HAND_LENGTH as f32) as i32;
    let hy = (CLOCK_CY as f32 - rad.cos() * HAND_LENGTH as f32) as i32;
    platform
        .draw_line(CLOCK_CX, CLOCK_CY, hx, hy, 0xFF2222, 4)
        .await;
}