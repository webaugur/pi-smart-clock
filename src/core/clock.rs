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

    platform
        .draw_rect(0, 0, layout.screen_w, layout.screen_h, 0x000000)
        .await;

    let diameter = (layout.clock_outer_r * 2) as u32;
    platform
        .draw_clock_face(layout.clock_cx, layout.clock_cy, diameter)
        .await;
    platform
        .draw_clock_second_hand(
            layout.clock_cx,
            layout.clock_cy,
            layout.hand_length,
            angle,
            is_night,
        )
        .await;
}