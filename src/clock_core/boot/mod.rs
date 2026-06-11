//! Multi-frame boot: splash + background module load + reveal transition.

mod loader;
pub mod reveal;
pub mod status;

use crate::clock_core::boot_splash::{SPLASH_FRAME_MS, SPLASH_MIN_MS};
use crate::drivers::platform::Platform;
use crate::runtime::SmartClockState;
use crate::runtime::UiMode;

#[cfg(feature = "full")]
use crate::clock_core::boot_splash::BootSplash;
#[cfg(feature = "full")]
use crate::platform::linux::SdlPlatformExt;

pub use loader::{status_for_step, BootLoaderProgress};
pub use reveal::REVEAL_FRAMES;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootPhase {
    Splash,
    Reveal,
}

pub struct BootController {
    pub phase: BootPhase,
    loader_step: u8,
    loader_done: bool,
    splash_frames: u32,
    pub reveal_frame: u32,
    pub status: &'static str,
    #[cfg(feature = "full")]
    splash: BootSplash,
    #[cfg(feature = "full")]
    splash_ready: bool,
}

impl BootController {
    pub fn new() -> Self {
        Self {
            phase: BootPhase::Splash,
            loader_step: 0,
            loader_done: false,
            splash_frames: 0,
            reveal_frame: 0,
            status: "Smart Clock",
            #[cfg(feature = "full")]
            splash: BootSplash::new(),
            #[cfg(feature = "full")]
            splash_ready: false,
        }
    }

    pub fn is_revealing(&self) -> bool {
        self.phase == BootPhase::Reveal
    }

    pub fn reveal_progress(&self) -> f32 {
        if self.phase != BootPhase::Reveal {
            return 0.0;
        }
        (self.reveal_frame as f32 / reveal::REVEAL_FRAMES as f32).min(1.0)
    }

    pub fn loader_progress(&self) -> BootLoaderProgress {
        let total = loader::STEP_COUNT;
        BootLoaderProgress {
            completed: self.loader_step.min(total),
            active: if !self.loader_done && self.loader_step < total {
                Some(self.loader_step)
            } else {
                None
            },
            total,
        }
    }

    pub fn splash_anim_frame(&self) -> u32 {
        self.splash_frames
    }

    #[cfg(feature = "full")]
    pub fn splash_mut(&mut self) -> &mut BootSplash {
        &mut self.splash
    }

    #[cfg(feature = "full")]
    pub(crate) fn ensure_splash(&mut self) {
        if self.splash_ready {
            return;
        }
        if !self.splash.try_start_video() {
            self.splash.try_load_image();
        }
        self.splash_ready = true;
    }

    #[cfg(feature = "full")]
    pub async fn render_splash_frame<P: Platform + SdlPlatformExt>(
        &mut self,
        platform: &mut P,
    ) {
        self.ensure_splash();
        if self.splash.has_frame() {
            self.splash.blit(platform.canvas_mut());
        }
        status::draw_boot_footer(
            platform,
            self.status,
            self.loader_progress(),
            self.splash_anim_frame(),
        )
        .await;
        platform.present().await;
    }

    #[cfg(not(feature = "full"))]
    pub async fn render_splash_frame<P: Platform>(&mut self, platform: &mut P) {
        platform.show_boot_splash(self.status).await;
        platform.present().await;
    }
}

pub async fn tick_boot<P: Platform>(state: &mut SmartClockState, platform: &mut P) {
    match state.boot.phase {
        BootPhase::Splash => {
            #[cfg(feature = "full")]
            state.boot.ensure_splash();

            state.boot.splash_frames = state.boot.splash_frames.saturating_add(1);

            if !state.boot.loader_done && state.boot.splash_frames % 2 == 0 {
                if state.boot.loader_step < loader::STEP_COUNT {
                    state.boot.status = loader::status_for_step(state.boot.loader_step);
                    loader::run_step(
                        state.boot.loader_step,
                        &mut state.alarms,
                        platform,
                    )
                    .await;
                    state.boot.loader_step = state.boot.loader_step.saturating_add(1);
                } else {
                    state.boot.loader_done = true;
                    state.boot.status = "Ready";
                }
            }

            let min_frames = SPLASH_MIN_MS.div_ceil(SPLASH_FRAME_MS).max(1) as u32;
            if state.boot.loader_done && state.boot.splash_frames >= min_frames {
                #[cfg(feature = "full")]
                {
                    state.boot.phase = BootPhase::Reveal;
                    state.boot.reveal_frame = 0;
                    state.boot.status = "Ready";
                    state.boot.splash.freeze_if_video();
                }
                #[cfg(not(feature = "full"))]
                {
                    state.boot_done = true;
                    state.ui_mode = UiMode::Clock;
                    platform.finish_boot().await;
                }
            }

            platform.delay(SPLASH_FRAME_MS).await;
        }
        #[cfg(feature = "full")]
        BootPhase::Reveal => {
            state.boot.reveal_frame = state.boot.reveal_frame.saturating_add(1);
            if state.boot.reveal_frame >= reveal::REVEAL_FRAMES {
                state.boot_done = true;
                state.ui_mode = UiMode::Clock;
                platform.finish_boot().await;
            }
            platform.delay(16).await;
        }
        #[cfg(not(feature = "full"))]
        BootPhase::Reveal => {
            state.boot_done = true;
            state.ui_mode = UiMode::Clock;
            platform.finish_boot().await;
        }
    }
}