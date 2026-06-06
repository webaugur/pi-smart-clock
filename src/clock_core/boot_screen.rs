use crate::drivers::ds3231::DS3231;
use crate::drivers::platform::Platform;
use crate::layout::l;

pub async fn show<P: Platform>(platform: &mut P) {
    let layout = l();
    let cx = layout.screen_w / 2;
    let cy = layout.screen_h / 2;
    platform.clear().await;
    platform
        .draw_text("Smart Clock", cx - 128, cy - 120, 50, 0x00FFAA)
        .await;
    platform
        .draw_text("Pico DVI + ESP8266", cx - 160, cy - 40, 28, 0x888888)
        .await;
    platform
        .draw_text("Waiting for RTC...", cx - 144, cy + 32, 28, 0xFFFF00)
        .await;
    platform.present().await;

    platform.delay(1000).await;

    DS3231::synchronize(platform).await;

    platform
        .draw_text("RTC Synced", cx - 112, cy + 96, 28, 0x00FF00)
        .await;
    platform.present().await;
    platform.delay(1000).await;

    platform.clear().await;
}