use crate::drivers::platform::Platform;
use crate::layout::SCREEN_W;

pub async fn show<P: Platform>(platform: &mut P) {
    platform.clear_center_area().await;

    platform
        .draw_text("Smart Clock", SCREEN_W / 2 - 128, 192, 50, 0x00FFAA)
        .await;
    platform
        .draw_text("Pico DVI + ESP8266", SCREEN_W / 2 - 160, 256, 26, 0x888888)
        .await;

    let version = env!("CARGO_PKG_VERSION");
    let git_hash = env!("GIT_FULL_HASH");

    platform
        .draw_text(
            &format!("Version: {}", version),
            SCREEN_W / 2 - 96,
            336,
            28,
            0xAAAAAA,
        )
        .await;
    platform
        .draw_text(&format!("Commit: {}", git_hash), 32, 400, 18, 0x666666)
        .await;

    platform
        .draw_text(
            "Built with Rust + Embassy",
            SCREEN_W / 2 - 160,
            480,
            22,
            0x555555,
        )
        .await;
    platform
        .draw_text(
            "© 2026 • David L Norris",
            SCREEN_W / 2 - 144,
            544,
            19,
            0x444444,
        )
        .await;

    platform
        .draw_text(
            "Press button to return",
            SCREEN_W / 2 - 144,
            640,
            22,
            0x888888,
        )
        .await;
}