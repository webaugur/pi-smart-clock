use crate::clock_core::alarm::AlarmManager;
use crate::prelude::*;
use crate::clock_core::alarm_ui::AlarmUI;
use crate::clock_core::alert_photos::AlertPhotoManager;
use crate::clock_core::alerts::AlertManager;
use crate::clock_core::energy_monitor::EnergyMonitor;
use crate::clock_core::logger::Logger;
use crate::clock_core::menu::MenuSystem;
use crate::clock_core::panels::weather::WeatherRadarPanel;
use crate::clock_core::sensors::EnvSensor;
use crate::clock_core::time_set_ui::TimeSetUI;
use crate::clock_core::update_scheduler::UpdateScheduler;
use crate::clock_core::weather::WeatherPanel as CoreWeatherPanel;
use crate::drivers::platform::Platform;
use crate::drivers::rotary_encoder::RotaryEncoder;
use crate::runtime::mode::UiMode;
use crate::runtime::tick;

#[cfg(feature = "linux-full")]
use crate::chimes::ChimeEngine;
#[cfg(feature = "linux-full")]
use crate::clock_core::alarm_video::AlarmVideoPlayer;
#[cfg(feature = "linux-full")]
use crate::modules::bar::BottomPanelBar;
#[cfg(feature = "linux-full")]
use crate::ota::updater::OtaUpdater;

#[cfg(not(feature = "linux-full"))]
pub struct OtaUpdater;

#[cfg(not(feature = "linux-full"))]
impl OtaUpdater {
    pub fn new() -> Self {
        Self
    }
}

pub struct SmartClockState {
    pub ui_mode: UiMode,
    pub boot_done: bool,
    pub alerts: AlertManager,
    pub alarms: AlarmManager,
    pub alarm_ui: AlarmUI,
    pub scheduler: UpdateScheduler,
    pub menu: MenuSystem,
    pub time_set: TimeSetUI,
    pub encoder: RotaryEncoder,
    pub sensors: EnvSensor,
    pub energy: EnergyMonitor,
    pub core_weather: CoreWeatherPanel,
    pub radar_panel: WeatherRadarPanel,
    pub photos: AlertPhotoManager,
    pub logger: Logger,
    pub ota: OtaUpdater,
    pub ringing_alarm: Option<usize>,
    #[cfg(feature = "linux-full")]
    pub chimes: ChimeEngine,
    #[cfg(feature = "linux-full")]
    pub bottom_panels: BottomPanelBar,
    #[cfg(feature = "linux-full")]
    pub alarm_video: AlarmVideoPlayer,
}

impl SmartClockState {
    pub fn new() -> Self {
        Self {
            ui_mode: UiMode::Boot,
            boot_done: false,
            alerts: AlertManager::new(),
            alarms: AlarmManager::new(),
            alarm_ui: AlarmUI::new(),
            scheduler: UpdateScheduler::new(),
            menu: MenuSystem::new(),
            time_set: TimeSetUI::new(),
            encoder: RotaryEncoder::new(),
            sensors: EnvSensor::new(),
            energy: EnergyMonitor::new(),
            core_weather: CoreWeatherPanel::new(),
            radar_panel: WeatherRadarPanel::new(),
            photos: AlertPhotoManager::new(),
            logger: Logger::new(),
            ota: OtaUpdater::new(),
            ringing_alarm: None,
            #[cfg(feature = "linux-full")]
            chimes: ChimeEngine::new(),
            #[cfg(feature = "linux-full")]
            bottom_panels: BottomPanelBar::new(),
            #[cfg(feature = "linux-full")]
            alarm_video: AlarmVideoPlayer::new(),
        }
    }

    #[cfg(feature = "linux-full")]
    pub async fn init<P: Platform + crate::platform::linux::SdlPlatformExt>(
        &mut self,
        _platform: &mut P,
    ) -> Result<(), String> {
        Ok(())
    }

    #[cfg(not(feature = "linux-full"))]
    pub async fn init<P: Platform>(&mut self, _platform: &mut P) -> Result<(), String> {
        Ok(())
    }

    #[cfg(feature = "linux-full")]
    pub async fn tick<P: Platform + crate::platform::linux::SdlPlatformExt>(
        &mut self,
        platform: &mut P,
    ) {
        tick::tick(self, platform).await;
    }

    #[cfg(not(feature = "linux-full"))]
    pub async fn tick<P: Platform>(&mut self, platform: &mut P) {
        tick::tick(self, platform).await;
    }

    #[cfg(feature = "linux-full")]
    pub async fn render_linux<P: Platform + crate::platform::linux::SdlPlatformExt>(
        &mut self,
        platform: &mut P,
    ) {
        tick::render_linux(self, platform).await;
    }
}