use crate::drivers::platform::Platform;
use crate::layout::l;

pub async fn update<P: Platform>(platform: &mut P) {
    let layout = l();
    let (hour, second) = current_hour_second(platform);
    let bounce = ((second as u32) % 8) as f32 * 0.35;
    let angle = (second * 6.0) + bounce;
    let is_night = hour >= 22 || hour < 6;

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

fn current_hour_second<P: Platform>(platform: &P) -> (u32, f32) {
    #[cfg(feature = "linux-full")]
    {
        use chrono::Timelike;
        let now = platform.get_current_time();
        return (now.hour(), now.second() as f32);
    }
    #[cfg(not(feature = "linux-full"))]
    {
        let now = platform.get_current_time();
        (now.hour, now.second as f32)
    }
}