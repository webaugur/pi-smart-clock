use crate::drivers::platform::Platform;

pub async fn show<P: Platform>(platform: &mut P) {
    platform.clear_center_area();

    platform.draw_text("Smart Clock", 260, 60, 32, 0x00FFAA);
    platform.draw_text("Pico 1 + DVI Sock + ESP8266", 180, 100, 16, 0x888888);

    platform.draw_text(&format!("Version: 0.2.0"), 280, 150, 18, 0xAAAAAA);
    platform.draw_text(&format!("Commit: {}", env!("GIT_FULL_HASH")), 140, 190, 12, 0x666666);

    platform.draw_text("Built with Rust + Embassy", 220, 240, 14, 0x555555);
    platform.draw_text("© 2026 • David L Norris", 240, 280, 12, 0x444444);

    platform.draw_text("Press button to return", 240, 340, 14, 0x888888);
}