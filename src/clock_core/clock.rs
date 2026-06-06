use crate::drivers::platform::Platform;
use crate::layout::l;

pub async fn update<P: Platform>(platform: &mut P) {
    let layout = l();
    let (hour, minute, second) = current_hms(platform);
    let bounce = ((second as u32) % 8) as f32 * 0.35;
    let second_angle = (second * 6.0) + bounce;
    let minute_angle = minute as f32 * 6.0 + second * 0.1;
    let hour_angle = (hour % 12) as f32 * 30.0 + minute as f32 * 0.5;
    let is_night = hour >= 22 || hour < 6;

    platform
        .draw_rect(0, 0, layout.screen_w, layout.screen_h, 0x000000)
        .await;

    let diameter = (layout.clock_outer_r * 2) as u32;
    platform
        .draw_clock_face(layout.clock_cx, layout.clock_cy, diameter)
        .await;

    let hour_len = layout.hand_length * 58 / 100;
    let minute_len = layout.hand_length * 82 / 100;

    platform
        .draw_clock_hour_hand(
            layout.clock_cx,
            layout.clock_cy,
            hour_len,
            hour_angle,
            is_night,
        )
        .await;
    platform
        .draw_clock_minute_hand(
            layout.clock_cx,
            layout.clock_cy,
            minute_len,
            minute_angle,
            is_night,
        )
        .await;
    platform
        .draw_clock_hub(layout.clock_cx, layout.clock_cy, is_night)
        .await;
    platform
        .draw_clock_second_hand(
            layout.clock_cx,
            layout.clock_cy,
            layout.hand_length,
            second_angle,
            is_night,
        )
        .await;
}

fn current_hms<P: Platform>(platform: &P) -> (u32, u32, f32) {
    #[cfg(feature = "linux-full")]
    {
        use chrono::Timelike;
        let now = platform.get_current_time();
        return (now.hour(), now.minute(), now.second() as f32);
    }
    #[cfg(not(feature = "linux-full"))]
    {
        let now = platform.get_current_time();
        (now.hour, now.minute, now.second as f32)
    }
}