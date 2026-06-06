use crate::clock_core::about;
use crate::drivers::platform::Platform;
use crate::runtime::mode::UiMode;
use crate::runtime::state::SmartClockState;

#[cfg(feature = "linux-full")]
use crate::clock as layout;
#[cfg(feature = "linux-full")]
use crate::clock_core::{boot, clock, status_bar};
#[cfg(feature = "linux-full")]
use crate::clock_core::boot::{reveal, status};
#[cfg(feature = "linux-full")]
use crate::layout::l;
#[cfg(feature = "linux-full")]
use crate::platform::linux::SdlPlatformExt;

#[cfg(feature = "linux-full")]
async fn render_clock_scene<P: Platform + SdlPlatformExt>(
    state: &mut SmartClockState,
    platform: &mut P,
) {
    platform.clear().await;
    let _ = layout::draw_layout_regions(platform.canvas_mut());

    clock::update(platform).await;
    state.radar_panel.update(platform, &state.alerts).await;

    state
        .bottom_panels
        .draw_backgrounds(platform.canvas_mut(), l());
    state.bottom_panels.draw_content(platform, l()).await;

    status_bar::draw(platform, state.sensors.temp_c, env!("GIT_HASH")).await;
}

#[cfg(feature = "linux-full")]
pub async fn render_boot<P: Platform + SdlPlatformExt>(
    state: &mut SmartClockState,
    platform: &mut P,
) {
    use crate::clock_core::boot::BootPhase;

    match state.boot.phase {
        BootPhase::Splash => state.boot.render_splash_frame(platform).await,
        BootPhase::Reveal => {
            render_clock_scene(state, platform).await;
            let progress = state.boot.reveal_progress();
            reveal::draw_checkerboard_splash(
                platform.canvas_mut(),
                state.boot.splash_mut(),
                progress,
            );
            if progress < 0.55 {
                status::draw_boot_footer(
                    platform,
                    state.boot.status,
                    state.boot.loader_progress(),
                    state.boot.splash_anim_frame(),
                )
                .await;
            }
            platform.present().await;
        }
    }
}

#[cfg(feature = "linux-full")]
pub async fn tick<P: Platform + SdlPlatformExt>(
    state: &mut SmartClockState,
    platform: &mut P,
) {
    if !state.boot_done {
        boot::tick_boot(state, platform).await;
        return;
    }

    state.encoder.update(platform).await;
    let _ = state.scheduler.tick(platform, &state.alerts).await;
    state.alerts.check_nws_alerts(platform).await;

    state.bottom_panels.tick(
        state.alerts.radar_active || state.alerts.amber_silver_active,
    );
    state.sensors.read(platform).await;
    state.alarms.check(platform, &mut state.ringing_alarm).await;

    if state.ui_mode == UiMode::Clock {
        state.chimes.tick(platform.audio_mut());
    }

    if state.ui_mode == UiMode::Clock && platform.read_pushbutton() {
        state.ui_mode = UiMode::Menu;
    }

    match state.ui_mode {
        UiMode::Menu => {
            state.menu.update(platform, &mut state.encoder).await;
            if state.menu.should_open_time_set() {
                state.ui_mode = UiMode::TimeSet;
                state.time_set.editing = true;
            } else if state.menu.should_open_about() {
                state.ui_mode = UiMode::About;
            } else if state.menu.should_close() {
                state.ui_mode = UiMode::Clock;
            }
        }
        UiMode::TimeSet => {
            state.time_set.update(platform, &mut state.encoder).await;
            if !state.time_set.editing {
                state.ui_mode = UiMode::Menu;
            }
        }
        UiMode::Clock => {
            if let Some(id) = state.ringing_alarm {
                if let Some(alarm) = state.alarms.alarms[id].clone() {
                    if !alarm.video_file.is_empty() {
                        state.alarm_video.start(&alarm.video_file);
                    }
                    state.alarm_ui.show(platform, &alarm).await;
                    state.ui_mode = UiMode::Alarm;
                }
            }
        }
        UiMode::Alarm => {
            state.alarm_video.poll_frame(platform.canvas_mut());
            if let Some(alarm) = state.alarm_ui.current_alarm.clone() {
                state.alarm_ui.draw_overlay(platform, &alarm).await;
            }
            if platform.read_pushbutton() {
                state.alarm_video.stop();
                platform.stop_alarm_sound().await;
                state.alarm_ui.hide(platform).await;
                state.ringing_alarm = None;
                state.ui_mode = UiMode::Clock;
            }
        }
        UiMode::About => {
            if platform.read_pushbutton() {
                state.ui_mode = UiMode::Clock;
            }
        }
        UiMode::Boot => state.ui_mode = UiMode::Clock,
    }

    state.core_weather.update(platform, &state.alerts).await;
}

#[cfg(not(feature = "linux-full"))]
use crate::clock_core::boot;

#[cfg(not(feature = "linux-full"))]
pub async fn tick<P: Platform>(state: &mut SmartClockState, platform: &mut P) {
    if !state.boot_done {
        boot::tick_boot(state, platform).await;
        platform.show_boot_splash(state.boot.status).await;
        platform.present().await;
        return;
    }

    state.encoder.update(platform).await;
    let _ = state.scheduler.tick(platform, &state.alerts).await;
    state.alerts.check_nws_alerts(platform).await;
    state.sensors.read(platform).await;
    state.alarms.check(platform, &mut state.ringing_alarm).await;

    if state.ui_mode == UiMode::Clock && platform.read_pushbutton() {
        state.ui_mode = UiMode::Menu;
    }

    match state.ui_mode {
        UiMode::Menu => {
            state.menu.update(platform, &mut state.encoder).await;
            if state.menu.should_open_time_set() {
                state.ui_mode = UiMode::TimeSet;
                state.time_set.editing = true;
            } else if state.menu.should_open_about() {
                state.ui_mode = UiMode::About;
            } else if state.menu.should_close() {
                state.ui_mode = UiMode::Clock;
            }
        }
        UiMode::TimeSet => {
            state.time_set.update(platform, &mut state.encoder).await;
            if !state.time_set.editing {
                state.ui_mode = UiMode::Menu;
            }
        }
        UiMode::Clock => {
            if let Some(id) = state.ringing_alarm {
                if let Some(alarm) = state.alarms.alarms[id].clone() {
                    state.alarm_ui.show(platform, &alarm).await;
                    state.ui_mode = UiMode::Alarm;
                }
            } else {
                crate::clock_core::clock::update(platform).await;
            }
        }
        UiMode::Alarm => {
            if let Some(alarm) = state.alarm_ui.current_alarm.clone() {
                state.alarm_ui.draw_overlay(platform, &alarm).await;
            }
            if platform.read_pushbutton() {
                platform.stop_alarm_sound().await;
                state.alarm_ui.hide(platform).await;
                state.ringing_alarm = None;
                state.ui_mode = UiMode::Clock;
            }
        }
        UiMode::About => {
            about::show(platform).await;
            if platform.read_pushbutton() {
                state.ui_mode = UiMode::Clock;
            }
        }
        UiMode::Boot => state.ui_mode = UiMode::Clock,
    }

    state.core_weather.update(platform, &state.alerts).await;
}

#[cfg(feature = "linux-full")]
pub async fn render_linux<P: Platform + SdlPlatformExt>(
    state: &mut SmartClockState,
    platform: &mut P,
) {
    if !state.boot_done {
        render_boot(state, platform).await;
        return;
    }

    platform.clear().await;
    let _ = layout::draw_layout_regions(platform.canvas_mut());

    let draw_clock_face = matches!(state.ui_mode, UiMode::Clock | UiMode::Alarm);
    if draw_clock_face {
        clock::update(platform).await;
        state.radar_panel.update(platform, &state.alerts).await;
    }

    if state.ui_mode == UiMode::Alarm {
        state.alarm_video.poll_frame(platform.canvas_mut());
    }

    let layout = l();
    if state.ui_mode != UiMode::Alarm {
        state
            .bottom_panels
            .draw_backgrounds(platform.canvas_mut(), layout);
        state.bottom_panels.draw_content(platform, layout).await;
    }

    status_bar::draw(platform, state.sensors.temp_c, env!("GIT_HASH")).await;

    match state.ui_mode {
        UiMode::Menu => state.menu.draw(platform).await,
        UiMode::About => about::show(platform).await,
        UiMode::Alarm => {
            if let Some(alarm) = state.alarm_ui.current_alarm.clone() {
                state.alarm_ui.draw_overlay(platform, &alarm).await;
            }
        }
        _ => {}
    }

    platform.present().await;
}