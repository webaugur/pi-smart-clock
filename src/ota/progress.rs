use crate::drivers::platform::Platform;

pub async fn show_progress<P: Platform>(platform: &mut P, percent: u8, stage: &str) {
    platform.clear_center_area();
    platform.draw_text("OTA Update", 300, 80, 28, 0x00FFAA);
    platform.draw_text(stage, 280, 130, 18, 0xAAAAAA);

    let bar_width = 400;
    let filled = (bar_width as f32 * percent as f32 / 100.0) as i32;

    platform.draw_rect(200, 200, bar_width, 24, 0x333333);
    platform.draw_rect(200, 200, filled, 24, 0x00FF00);

    platform.draw_text(&format!("{}%", percent), 380, 205, 16, 0xFFFFFF);
    platform.present();
}