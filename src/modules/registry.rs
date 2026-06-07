use sdl2::render::Canvas;
use sdl2::video::Window;

use super::bottom_module::{BottomModule, PanelLine};
use super::calendar::CalendarPanel;
use super::holidays::HolidaysPanel;
use super::lunar::LunarPanel;
use super::module_id::ModuleId;
use super::venus_pentagram::VenusMoonPentagramPanel;
use super::weather::WeatherPanel;
use super::zodiac::ZodiacPanel;

pub struct ModuleRegistry {
    weather: WeatherPanel,
    calendar: CalendarPanel,
    holidays: HolidaysPanel,
    // New upper-row modules (same BottomModule format)
    lunar: LunarPanel,
    zodiac: ZodiacPanel,
    pentagram: VenusMoonPentagramPanel,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            weather: WeatherPanel::new(),
            calendar: CalendarPanel::new(),
            holidays: HolidaysPanel::new(),
            lunar: LunarPanel::new(),
            zodiac: ZodiacPanel::new(),
            pentagram: VenusMoonPentagramPanel::new(),
        }
    }

    pub fn weather_mut(&mut self) -> &mut WeatherPanel {
        &mut self.weather
    }

    pub fn draw_background(
        &mut self,
        module: ModuleId,
        canvas: &mut Canvas<Window>,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) {
        self.module_mut(module)
            .draw_background(canvas, x, y, w, h);
    }

    pub fn title(&self, module: ModuleId) -> (String, u32) {
        self.module(module).title()
    }

    pub fn lines(&self, module: ModuleId) -> Vec<PanelLine> {
        self.module(module).lines()
    }

    pub fn tick(&mut self, module: ModuleId, alerts_active: bool) {
        self.module_mut(module).tick(alerts_active);
    }

    fn module(&self, id: ModuleId) -> &dyn BottomModule {
        match id {
            ModuleId::Weather => &self.weather,
            ModuleId::Calendar => &self.calendar,
            ModuleId::Holidays => &self.holidays,
            ModuleId::Lunar => &self.lunar,
            ModuleId::Zodiac => &self.zodiac,
            ModuleId::Pentagram => &self.pentagram,
        }
    }

    fn module_mut(&mut self, id: ModuleId) -> &mut dyn BottomModule {
        match id {
            ModuleId::Weather => &mut self.weather,
            ModuleId::Calendar => &mut self.calendar,
            ModuleId::Holidays => &mut self.holidays,
            ModuleId::Lunar => &mut self.lunar,
            ModuleId::Zodiac => &mut self.zodiac,
            ModuleId::Pentagram => &mut self.pentagram,
        }
    }
}