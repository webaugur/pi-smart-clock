use crate::drivers::platform::Platform;

pub async fn show<P: Platform>(platform: &mut P) {
    platform.clear_center_area().await;

    platform.draw_text("Smart Clock", 260, 60, 32, 0x00FFAA).await;
    platform
        .draw_text("Pico 1 + DVI Sock + ESP8266", 180, 100, 16, 0x888888)
        .await;

    let version = env!("CARGO_PKG_VERSION");
    let git_hash = env!("GIT_FULL_HASH");

    platform
        .draw_text(&format!("Version: {}", version), 280, 150, 18, 0xAAAAAA)
        .await;
    platform
        .draw_text(&format!("Commit: {}", git_hash), 140, 190, 11, 0x666666)
        .await;

    platform
        .draw_text("Built with Rust + Embassy", 220, 240, 14, 0x555555)
        .await;
    platform
        .draw_text("© 2026 • David L Norris", 240, 280, 12, 0x444444)
        .await;

    platform
        .draw_text("Press button to return", 240, 340, 14, 0x888888)
        .await;
}