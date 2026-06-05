use chrono::Local;

use crate::core::{about, persistence, status_bar};
use crate::core::clock;
use crate::drivers::platform::Platform;
use crate::platform::linux::SdlPlatformExt;
use crate::runtime::mode::UiMode;
use crate::runtime::state::SmartClockState;

#[cfg(feature = "linux-full")]
use crate::clock as layout;
#[cfg(feature = "linux-full")]
use crate::layout::l;
#[cfg(feature = "linux-full")]
use crate::panel::Panel;

pub async fn tick<P: Platform + SdlPlatformExt>(
    state: &mut SmartClockState,
    platform: &mut P,
) {
    if !state.boot_done {
        crate::core::boot_screen::show(platform).await;
        persistence::load_alarms(platform, &mut state.alarms).await;
        state.boot_done = true;
        state.ui_mode = UiMode::Clock;
        return;
    }

    state.encoder.update(platform).await;
    if let Some((temp, cond)) = state.scheduler.tick(platform, &state.alerts).await {
        #[cfg(feature = "linux-full")]
        state.weather_panel.set_weather(temp, cond);
    }
    state.alerts.check_nws_alerts(platform).await;
    state.sensors.read(platform).await;
    state.alarms.check(platform, &mut state.ringing_alarm).await;

    #[cfg(feature = "linux-full")]
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
                    #[cfg(feature = "linux-full")]
                    if !alarm.video_file.is_empty() {
                        state.alarm_video.start(&alarm.video_file);
                    }
                    state.alarm_ui.show(platform, &alarm).await;
                    state.ui_mode = UiMode::Alarm;
                }
            }
        }
        UiMode::Alarm => {
            #[cfg(feature = "linux-full")]
            state.alarm_video.poll_frame(platform.canvas_mut());
            if let Some(alarm) = state.alarm_ui.current_alarm.clone() {
                state.alarm_ui.draw_overlay(platform, &alarm).await;
            }
            if platform.read_pushbutton() {
                #[cfg(feature = "linux-full")]
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

pub async fn render_linux<P: Platform + SdlPlatformExt>(
    state: &mut SmartClockState,
    platform: &mut P,
) {
    platform.clear().await;
    let _ = layout::draw_layout_regions(platform.canvas_mut());

    let draw_clock_face = matches!(state.ui_mode, UiMode::Clock | UiMode::Alarm);
    if draw_clock_face {
        clock::update(platform).await;
        state.radar_panel.update(platform, &state.alerts).await;
        if let Some(font) = platform.font() {
            let _ = layout::draw_roman_numerals(platform.canvas_mut(), font, Local::now());
        }
    }

    #[cfg(feature = "linux-full")]
    if state.ui_mode == UiMode::Alarm {
        state.alarm_video.poll_frame(platform.canvas_mut());
    }

    let layout = l();
    if state.ui_mode != UiMode::Alarm {
        state.weather_panel.draw(
            platform.canvas_mut(),
            layout.center_x,
            layout.center_y,
            layout.center_w as i32,
            layout.center_h as i32,
        );
        platform
            .draw_text(
                &format!("{}°", state.weather_panel.temp()),
                layout.center_x + 24,
                layout.center_y + 32,
                32,
                0xFFFFFF,
            )
            .await;
        platform
            .draw_text(
                state.weather_panel.condition(),
                layout.center_x + 24,
                layout.center_y + 80,
                26,
                0xAAAAAA,
            )
            .await;
    }

    state.calendar_panel.draw(
        platform.canvas_mut(),
        layout.cal_x,
        layout.cal_y,
        layout.cal_w,
        layout.cal_h,
    );
    state.holidays_panel.draw(
        platform.canvas_mut(),
        layout.hol_x,
        layout.hol_y,
        layout.hol_w,
        layout.hol_h,
    );

    platform
        .draw_text(
            "Calendar",
            layout.cal_x + 16,
            layout.cal_y + 8,
            22,
            0x88AAFF,
        )
        .await;
    for (i, ev) in state.calendar_panel.events.iter().take(3).enumerate() {
        platform
            .draw_text(
                ev,
                layout.cal_x + 16,
                layout.cal_y + 48 + (i as i32) * 35,
                19,
                0xCCCCCC,
            )
            .await;
    }
    platform
        .draw_text(
            "Holidays",
            layout.hol_x + 16,
            layout.hol_y + 8,
            22,
            0xFFAA88,
        )
        .await;
    for (i, h) in state.holidays_panel.holidays.iter().take(3).enumerate() {
        platform
            .draw_text(
                h,
                layout.hol_x + 16,
                layout.hol_y + 48 + (i as i32) * 35,
                19,
                0xCCCCCC,
            )
            .await;
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