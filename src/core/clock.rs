use crate::drivers::platform::Platform;
use crate::layout::l;
use chrono::{Local, Timelike};

pub async fn update<P: Platform>(platform: &mut P) {
    let layout = l();
    let now = Local::now();
    let seconds = now.second() as f32;
    let bounce = (seconds.fract() * 8.0).sin().abs() * 3.0;
    let angle = (seconds * 6.0) + bounce;

    let is_night = now.hour() >= 22 || now.hour() < 6;
    let bg = if is_night { 0x0A0A0A } else { 0x000000 };
    platform
        .draw_rect(0, 0, layout.screen_w, layout.screen_h, bg)
        .await;

    platform
        .draw_circle(
            layout.clock_cx,
            layout.clock_cy,
            layout.clock_outer_r,
            0xFFFFFF,
        )
        .await;
    platform
        .draw_circle(
            layout.clock_cx,
            layout.clock_cy,
            layout.clock_inner_r,
            if is_night { 0x1A1400 } else { 0x111111 },
        )
        .await;

    let amber = if is_night { 0xFFAA33 } else { 0xFFFFFF };
    for i in 0..12 {
        let ang = (i as f32 * 30.0).to_radians();
        let x1 = (layout.clock_cx as f32 + ang.sin() * layout.tick_outer_r as f32) as i32;
        let y1 = (layout.clock_cy as f32 - ang.cos() * layout.tick_outer_r as f32) as i32;
        let x2 = (layout.clock_cx as f32 + ang.sin() * layout.tick_inner_r as f32) as i32;
        let y2 = (layout.clock_cy as f32 - ang.cos() * layout.tick_inner_r as f32) as i32;
        platform.draw_line(x1, y1, x2, y2, amber, 2).await;
    }

    let rad = angle.to_radians();
    let hx = (layout.clock_cx as f32 + rad.sin() * layout.hand_length as f32) as i32;
    let hy = (layout.clock_cy as f32 - rad.cos() * layout.hand_length as f32) as i32;
    platform
        .draw_line(layout.clock_cx, layout.clock_cy, hx, hy, 0xFF2222, 4)
        .await;
}