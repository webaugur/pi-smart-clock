#[cfg(feature = "full")]
use crate::clock_core::boot::loader::{self, BootLoaderProgress};
#[cfg(feature = "full")]
use crate::drivers::platform::Platform;
#[cfg(feature = "full")]
use crate::layout::{l, Layout};

/// Segmented boot loader bar; set `false` for quieter release builds.
pub const SHOW_BOOT_PROGRESS: bool = true;

/// Status line and module labels share one readable size.
pub const BOOT_FONT_SIZE: u8 = 44;

#[cfg(feature = "full")]
pub async fn draw_boot_footer<P: Platform>(
    platform: &mut P,
    status: &str,
    progress: BootLoaderProgress,
    anim_frame: u32,
) {
    if SHOW_BOOT_PROGRESS {
        draw_boot_progress_bar(platform, progress, anim_frame).await;
    }
    draw_boot_status(platform, status, BOOT_FONT_SIZE, 0x00FFAA).await;
}

#[cfg(feature = "full")]
async fn draw_boot_progress_bar<P: Platform>(
    platform: &mut P,
    progress: BootLoaderProgress,
    anim_frame: u32,
) {
    let layout = l();
    let label_size = BOOT_FONT_SIZE;
    let bar_h = 10i32;
    let gap = 8i32;

    // Bottom → top: module labels, bar (status is drawn above the bar separately).
    let label_y = layout.screen_h - i32::from(label_size) - 12;
    let bar_y = label_y - bar_h - gap;

    let margin = layout.screen_w / 10;
    let bar_w = layout.screen_w - margin * 2;
    let n = i32::from(progress.total.max(1));
    let total_gaps = gap * (n - 1);
    let seg_w = (bar_w - total_gaps) / n;

    for i in 0..progress.total {
        let x = margin + i32::from(i) * (seg_w + gap);
        let color = segment_color(i, progress, anim_frame);
        platform.draw_rect(x, bar_y, seg_w, bar_h, color).await;

        let label = loader::short_label_for_step(i);
        let label_w = (label.len() as i32).saturating_mul(i32::from(label_size) / 2 + 2);
        let label_x = x + (seg_w - label_w) / 2;
        platform
            .draw_text(label, label_x, label_y, label_size, color)
            .await;
    }
}

#[cfg(feature = "full")]
fn segment_color(i: u8, progress: BootLoaderProgress, anim_frame: u32) -> u32 {
    if i < progress.completed {
        return 0x00AA66;
    }
    if progress.active == Some(i) {
        if (anim_frame / 6) % 2 == 0 {
            0x00FFAA
        } else {
            0x006644
        }
    } else {
        0x333333
    }
}

#[cfg(feature = "full")]
pub async fn draw_boot_status<P: Platform>(platform: &mut P, text: &str, size: u8, color: u32) {
    let layout = l();
    let x = boot_status_x(text, size, &layout);
    let y = boot_status_y(text, size, &layout);
    platform.draw_text(text, x, y, size, color).await;
}

/// Status line sits above the progress bar block.
#[cfg(feature = "full")]
fn boot_status_y(_text: &str, size: u8, layout: &Layout) -> i32 {
    let bar_h = 10i32;
    let gap = 8i32;
    let label_y = layout.screen_h - i32::from(BOOT_FONT_SIZE) - 12;
    let bar_y = label_y - bar_h - gap;
    bar_y - i32::from(size) - gap
}

#[cfg(feature = "full")]
fn boot_status_x(text: &str, size: u8, layout: &Layout) -> i32 {
    let approx_w = (text.len() as i32).saturating_mul(i32::from(size) / 2 + 4);
    (layout.screen_w - approx_w) / 2
}