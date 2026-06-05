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
use crate::panel::Panel;

pub async fn tick<P: Platform>(state: &mut SmartClockState, platform: &mut P) {
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
        state.chimes.tick(platform).await;
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
                    state.alarm_ui.show(platform, &alarm).await;
                    state.ui_mode = UiMode::Alarm;
                }
            }
        }
        UiMode::Alarm => {
            if platform.read_pushbutton() {
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

    if matches!(state.ui_mode, UiMode::Clock | UiMode::Alarm) {
        clock::update(platform).await;
        state.radar_panel.update(platform, &state.alerts).await;
        if let Some(font) = platform.font() {
            let _ = layout::draw_roman_numerals(platform.canvas_mut(), font, Local::now());
        }
    }

    state.weather_panel
        .draw(platform.canvas_mut(), 270, 145, 260, 150);
    state.calendar_panel
        .draw(platform.canvas_mut(), 20, 330, 240, 140);
    state.holidays_panel
        .draw(platform.canvas_mut(), 540, 330, 240, 140);

    platform
        .draw_text(
            &format!("{}°", state.weather_panel.temp()),
            285,
            165,
            20,
            0xFFFFFF,
        )
        .await;
    platform
        .draw_text(state.weather_panel.condition(), 285, 195, 16, 0xAAAAAA)
        .await;

    status_bar::draw(platform, state.sensors.temp_c, env!("GIT_HASH")).await;

    match state.ui_mode {
        UiMode::Menu => state.menu.draw(platform).await,
        UiMode::About => about::show(platform).await,
        _ => {}
    }

    platform.present().await;
}
