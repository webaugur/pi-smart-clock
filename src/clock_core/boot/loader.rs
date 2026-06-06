//! Incremental startup work — one step per splash frame so the UI stays responsive.

use crate::clock_core::alarm::AlarmManager;
use crate::clock_core::persistence;
use crate::drivers::ds3231::DS3231;
use crate::drivers::platform::Platform;

#[cfg(feature = "linux-full")]
use crate::icons;
#[cfg(feature = "linux-full")]
use crate::modules::faces;

pub const STEP_COUNT: u8 = 4;

/// Loader state for the boot progress bar (which step finished / is running).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootLoaderProgress {
    pub completed: u8,
    pub active: Option<u8>,
    pub total: u8,
}

#[cfg(feature = "linux-full")]
pub fn short_label_for_step(step: u8) -> &'static str {
    match step {
        0 => "RTC",
        1 => "Face",
        2 => "Icons",
        3 => "Alarms",
        _ => "",
    }
}

pub fn status_for_step(step: u8) -> &'static str {
    match step {
        0 => "Syncing time...",
        1 => "Loading clock face...",
        2 => "Loading icons...",
        3 => "Loading alarms...",
        _ => "Ready",
    }
}

pub async fn run_step<P: Platform>(
    step: u8,
    alarms: &mut AlarmManager,
    platform: &mut P,
) {
    match step {
        0 => DS3231::synchronize(platform).await,
        1 => {
            #[cfg(feature = "linux-full")]
            faces::preload_active_face();
        }
        2 => {
            #[cfg(feature = "linux-full")]
            icons::preload();
        }
        3 => persistence::load_alarms(platform, alarms).await,
        _ => {}
    }
}