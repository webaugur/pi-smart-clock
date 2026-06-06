use crate::drivers::platform::Platform;
use crate::layout::l;

pub async fn show<P: Platform>(platform: &mut P) {
    let layout = l();
    let cx = layout.screen_w / 2;
    platform.clear_center_area().await;

    platform
        .draw_text("Smart Clock", cx - 128, layout.center_y - 80, 50, 0x00FFAA)
        .await;
    platform
        .draw_text("Pico DVI + ESP8266", cx - 160, layout.center_y - 16, 26, 0x888888)
        .await;

    let version = env!("CARGO_PKG_VERSION");
    let git_hash = env!("GIT_FULL_HASH");

    platform
        .draw_text(
            &format!("Version: {}", version),
            cx - 96,
            layout.center_y + 64,
            28,
            0xAAAAAA,
        )
        .await;
    platform
        .draw_text(&format!("Commit: {}", git_hash), 32, layout.center_y + 128, 18, 0x666666)
        .await;

    platform
        .draw_text(
            "Built with Rust + Embassy",
            cx - 160,
            layout.center_y + 208,
            22,
            0x555555,
        )
        .await;
    platform
        .draw_text(
            "© 2026 • David L Norris",
            cx - 144,
            layout.center_y + 272,
            19,
            0x444444,
        )
        .await;

    platform
        .draw_text(
            "Press button to return",
            cx - 144,
            layout.center_y + 368,
            22,
            0x888888,
        )
        .await;
}