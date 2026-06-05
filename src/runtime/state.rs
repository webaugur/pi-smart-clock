use crate::core::alarm::{AlarmManager};
use crate::core::alarm_ui::AlarmUI;
use crate::core::alert_photos::AlertPhotoManager;
use crate::core::alerts::AlertManager;
use crate::core::energy_monitor::EnergyMonitor;
use crate::core::logger::Logger;
use crate::core::menu::MenuSystem;
use crate::core::panels::weather::WeatherRadarPanel;
use crate::core::sensors::EnvSensor;
use crate::core::time_set_ui::TimeSetUI;
use crate::core::update_scheduler::UpdateScheduler;
use crate::core::weather::WeatherPanel as CoreWeatherPanel;
use crate::drivers::platform::Platform;
use crate::drivers::rotary_encoder::RotaryEncoder;
use crate::ota::updater::OtaUpdater;
use crate::runtime::mode::UiMode;
use crate::runtime::tick;

#[cfg(feature = "linux-full")]
use crate::chimes::ChimeEngine;
#[cfg(feature = "linux-full")]
use crate::core::alarm_video::AlarmVideoPlayer;
#[cfg(feature = "linux-full")]
use crate::modules::calendar::CalendarPanel;
#[cfg(feature = "linux-full")]
use crate::modules::holidays::HolidaysPanel;
#[cfg(feature = "linux-full")]
use crate::modules::weather::WeatherPanel;

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
    pub weather_panel: WeatherPanel,
    #[cfg(feature = "linux-full")]
    pub calendar_panel: CalendarPanel,
    #[cfg(feature = "linux-full")]
    pub holidays_panel: HolidaysPanel,
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
            weather_panel: WeatherPanel::new(),
            #[cfg(feature = "linux-full")]
            calendar_panel: CalendarPanel::new(),
            #[cfg(feature = "linux-full")]
            holidays_panel: HolidaysPanel::new(),
            #[cfg(feature = "linux-full")]
            alarm_video: AlarmVideoPlayer::new(),
        }
    }

    pub async fn init<P: Platform + crate::platform::linux::SdlPlatformExt>(
        &mut self,
        _platform: &mut P,
    ) -> Result<(), String> {
        Ok(())
    }

    pub async fn tick<P: Platform + crate::platform::linux::SdlPlatformExt>(
        &mut self,
        platform: &mut P,
    ) {
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
