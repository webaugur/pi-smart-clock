use crate::drivers::platform::Platform;
use crate::layout::SCREEN_W;

pub async fn show<P: Platform>(platform: &mut P) {
    platform.clear_center_area().await;

    platform
        .draw_text("Smart Clock", SCREEN_W / 2 - 80, 120, 32, 0x00FFAA)
        .await;
    platform
        .draw_text("Pico DVI + ESP8266", SCREEN_W / 2 - 100, 160, 16, 0x888888)
        .await;

    let version = env!("CARGO_PKG_VERSION");
    let git_hash = env!("GIT_FULL_HASH");

    platform
        .draw_text(&format!("Version: {}", version), SCREEN_W / 2 - 60, 210, 18, 0xAAAAAA)
        .await;
    platform
        .draw_text(&format!("Commit: {}", git_hash), 20, 250, 11, 0x666666)
        .await;

    platform
        .draw_text("Built with Rust + Embassy", SCREEN_W / 2 - 100, 300, 14, 0x555555)
        .await;
    platform
        .draw_text("© 2026 • David L Norris", SCREEN_W / 2 - 90, 340, 12, 0x444444)
        .await;

    platform
        .draw_text("Press button to return", SCREEN_W / 2 - 90, 400, 14, 0x888888)
        .await;
}