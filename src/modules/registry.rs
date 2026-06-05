use sdl2::render::Canvas;
use sdl2::video::Window;

use super::bottom_module::{BottomModule, PanelLine};
use super::calendar::CalendarPanel;
use super::module_id::ModuleId;
use super::holidays::HolidaysPanel;
use super::weather::WeatherPanel;

pub struct ModuleRegistry {
    weather: WeatherPanel,
    calendar: CalendarPanel,
    holidays: HolidaysPanel,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            weather: WeatherPanel::new(),
            calendar: CalendarPanel::new(),
            holidays: HolidaysPanel::new(),
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
        }
    }

    fn module_mut(&mut self, id: ModuleId) -> &mut dyn BottomModule {
        match id {
            ModuleId::Weather => &mut self.weather,
            ModuleId::Calendar => &mut self.calendar,
            ModuleId::Holidays => &mut self.holidays,
        }
    }
}