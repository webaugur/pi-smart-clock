use crate::drivers::ds3231::DS3231;
use crate::drivers::platform::Platform;
use crate::layout::SCREEN_W;

pub async fn show<P: Platform>(platform: &mut P) {
    platform.clear().await;
    platform
        .draw_text("Smart Clock", SCREEN_W / 2 - 128, 320, 50, 0x00FFAA)
        .await;
    platform
        .draw_text("Pico DVI + ESP8266", SCREEN_W / 2 - 160, 400, 28, 0x888888)
        .await;
    platform
        .draw_text("Waiting for RTC...", SCREEN_W / 2 - 144, 512, 28, 0xFFFF00)
        .await;
    platform.present().await;

    platform.delay(1000).await;

    DS3231::synchronize(platform).await;

    platform
        .draw_text("RTC Synced", SCREEN_W / 2 - 112, 608, 28, 0x00FF00)
        .await;
    platform.present().await;
    platform.delay(1000).await;

    platform.clear().await;
}