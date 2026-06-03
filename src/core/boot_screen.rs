use crate::drivers::platform::Platform;
use crate::drivers::ds3231::DS3231;
use embassy_time::Timer;

pub async fn show<P: Platform>(platform: &mut P) {
    platform.clear();
    platform.draw_text("Smart Clock", 260, 80, 32, 0x00FFAA);
    platform.draw_text("Pico DVI + ESP8266", 200, 130, 18, 0x888888);
    platform.draw_text("Waiting for RTC...", 240, 200, 18, 0xFFFF00);
    platform.present();

    Timer::after_secs(1).await;

    // Try to sync DS3231
    DS3231::synchronize(platform).await;

    platform.draw_text("✅ RTC Synced", 260, 260, 18, 0x00FF00);
    platform.present();
    Timer::after_secs(1).await;

    platform.clear();
}