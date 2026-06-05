use crate::drivers::ds3231::DS3231;
use crate::drivers::platform::Platform;

pub async fn show<P: Platform>(platform: &mut P) {
    platform.clear().await;
    platform.draw_text("Smart Clock", 260, 80, 32, 0x00FFAA).await;
    platform
        .draw_text("Pico DVI + ESP8266", 200, 130, 18, 0x888888)
        .await;
    platform
        .draw_text("Waiting for RTC...", 240, 200, 18, 0xFFFF00)
        .await;
    platform.present().await;

    platform.delay(1000).await;

    DS3231::synchronize(platform).await;

    platform.draw_text("RTC Synced", 260, 260, 18, 0x00FF00).await;
    platform.present().await;
    platform.delay(1000).await;

    platform.clear().await;
}