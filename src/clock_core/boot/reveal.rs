//! Splash → clock transition (checkerboard dissolve).

#[cfg(feature = "linux-full")]
use sdl2::render::Canvas;
#[cfg(feature = "linux-full")]
use sdl2::video::Window;

#[cfg(feature = "linux-full")]
use crate::clock_core::boot_splash::BootSplash;

pub const REVEAL_FRAMES: u32 = 48;
pub const CHECKER_BLOCK: i32 = 32;

/// Even checker cells dissolve by 50%, odd cells by 100%.
pub fn cell_shows_splash(cx: i32, cy: i32, progress: f32) -> bool {
    let progress = progress.clamp(0.0, 1.0);
    if progress >= 1.0 {
        return false;
    }
    let threshold = if (cx + cy) % 2 == 0 { 0.5 } else { 1.0 };
    progress < threshold
}

#[cfg(feature = "linux-full")]
pub fn draw_checkerboard_splash(canvas: &mut Canvas<Window>, splash: &BootSplash, progress: f32) {
    let progress = progress.clamp(0.0, 1.0);
    if progress >= 1.0 {
        return;
    }
    splash.blit_reveal_checkerboard(canvas, progress);
}