use crate::clock_core::boot_splash::{SPLASH_FRAME_MS, SPLASH_MIN_MS};
use crate::drivers::ds3231::DS3231;
use crate::drivers::platform::Platform;

#[cfg(feature = "linux-full")]
use crate::clock_core::boot_splash::BootSplash;
#[cfg(feature = "linux-full")]
use crate::layout::{l, Layout};
#[cfg(feature = "linux-full")]
use crate::platform::linux::SdlPlatformExt;

#[cfg(feature = "linux-full")]
async fn draw_boot_status<P: Platform>(platform: &mut P, text: &str, size: u8, color: u32) {
    let layout = l();
    let x = boot_status_x(text, size, &layout);
    let y = boot_status_y(size, &layout);
    platform.draw_text(text, x, y, size, color).await;
}

#[cfg(feature = "linux-full")]
fn boot_status_y(size: u8, layout: &Layout) -> i32 {
    layout.screen_h - i32::from(size) - 20
}

#[cfg(feature = "linux-full")]
fn boot_status_x(text: &str, size: u8, layout: &Layout) -> i32 {
    let approx_w = (text.len() as i32).saturating_mul(i32::from(size) / 2 + 4);
    (layout.screen_w - approx_w) / 2
}

#[cfg(feature = "linux-full")]
async fn present_splash_frame<P: Platform + SdlPlatformExt>(
    platform: &mut P,
    splash: &mut BootSplash,
    status: &str,
    size: u8,
    color: u32,
) {
    if splash.has_frame() {
        splash.blit(platform.canvas_mut());
    }
    draw_boot_status(platform, status, size, color).await;
    platform.present().await;
}

#[cfg(feature = "linux-full")]
pub async fn show<P: Platform + SdlPlatformExt>(platform: &mut P) {
    platform.clear().await;

    let mut splash = BootSplash::new();
    if !splash.try_start_video() {
        splash.try_load_image();
    }

    if splash.is_active() {
        let frames = SPLASH_MIN_MS.div_ceil(SPLASH_FRAME_MS).max(1);
        for _ in 0..frames {
            present_splash_frame(platform, &mut splash, "Smart Clock", 28, 0x00FFAA).await;
            platform.delay(SPLASH_FRAME_MS).await;
        }

        present_splash_frame(platform, &mut splash, "Syncing time...", 28, 0xFFFF00).await;
        DS3231::synchronize(platform).await;

        present_splash_frame(platform, &mut splash, "Ready", 28, 0x00FF00).await;
        platform.delay(800).await;
        splash.stop();
    } else {
        draw_boot_status(platform, "Smart Clock", 50, 0x00FFAA).await;
        platform.present().await;
        platform.delay(600).await;
        draw_boot_status(platform, "Syncing time...", 28, 0xFFFF00).await;
        platform.present().await;
        DS3231::synchronize(platform).await;
        draw_boot_status(platform, "Ready", 28, 0x00FF00).await;
        platform.present().await;
        platform.delay(800).await;
    }

    platform.clear().await;
    platform.finish_boot().await;
}

#[cfg(not(feature = "linux-full"))]
pub async fn show<P: Platform>(platform: &mut P) {
    platform.clear().await;

    let frames = SPLASH_MIN_MS.div_ceil(SPLASH_FRAME_MS).max(1);
    for _ in 0..frames {
        platform.show_boot_splash("Smart Clock").await;
        platform.present().await;
        platform.delay(SPLASH_FRAME_MS).await;
    }

    platform.show_boot_splash("Syncing time...").await;
    platform.present().await;
    DS3231::synchronize(platform).await;

    platform.show_boot_splash("Ready").await;
    platform.present().await;
    platform.delay(800).await;
    platform.clear().await;
    platform.finish_boot().await;
}