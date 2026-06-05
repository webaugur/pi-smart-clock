use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::drivers::platform::Platform;
use crate::layout::Layout;
use crate::modules::calendar::CalendarPanel;
use crate::modules::holidays::HolidaysPanel;
use crate::modules::kind::PanelKind;
use crate::modules::weather::WeatherPanel;
use crate::panel::Panel;

pub struct PanelLine {
    pub text: String,
    /// 0 = use layout `bottom_body_pt`
    pub size_pt: u8,
}

pub struct BottomPanelBar {
    pub slots: [PanelKind; 3],
    weather: WeatherPanel,
    calendar: CalendarPanel,
    holidays: HolidaysPanel,
}

impl BottomPanelBar {
    pub fn new() -> Self {
        Self::with_slots(crate::modules::config::load_panel_slots())
    }

    pub fn with_slots(slots: [PanelKind; 3]) -> Self {
        Self {
            slots,
            weather: WeatherPanel::new(),
            calendar: CalendarPanel::new(),
            holidays: HolidaysPanel::new(),
        }
    }

    pub fn weather_mut(&mut self) -> &mut WeatherPanel {
        &mut self.weather
    }

    pub fn draw_backgrounds(&mut self, canvas: &mut Canvas<Window>, layout: &Layout) {
        let slots = self.slots;
        for (slot, kind) in slots.iter().enumerate() {
            let (x, y, w, h) = layout.panel_slot(slot);
            self.draw_background(canvas, *kind, x, y, w, h);
        }
    }

    pub async fn draw_content<P: Platform>(&mut self, platform: &mut P, layout: &Layout) {
        let pad = 10;
        let body_y = layout.bottom_y + 40;

        let slots = self.slots;
        for (slot, kind) in slots.iter().enumerate() {
            let (x, _, _, _) = layout.panel_slot(slot);
            let (title, title_color) = self.title(*kind);
            platform
                .draw_text(
                    &title,
                    x + pad,
                    layout.bottom_y + 6,
                    layout.bottom_title_pt,
                    title_color,
                )
                .await;

            let lines = self.lines(*kind);
            for (i, line) in lines.into_iter().enumerate() {
                let size = if line.size_pt == 0 {
                    layout.bottom_body_pt
                } else {
                    line.size_pt
                };
                platform
                    .draw_text(
                        &line.text,
                        x + pad,
                        body_y + (i as i32) * layout.bottom_line_gap,
                        size,
                        0xCCCCCC,
                    )
                    .await;
            }
        }
    }

    fn draw_background(
        &mut self,
        canvas: &mut Canvas<Window>,
        kind: PanelKind,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) {
        match kind {
            PanelKind::Weather => self.weather.draw(canvas, x, y, w, h),
            PanelKind::Calendar => self.calendar.draw(canvas, x, y, w, h),
            PanelKind::Holidays => self.holidays.draw(canvas, x, y, w, h),
        }
    }

    fn title(&self, kind: PanelKind) -> (String, u32) {
        match kind {
            PanelKind::Weather => (self.weather.panel_title(), 0x00FFAA),
            PanelKind::Calendar => ("Calendar".to_string(), 0x88AAFF),
            PanelKind::Holidays => ("Holidays".to_string(), 0xFFAA88),
        }
    }

    fn lines(&self, kind: PanelKind) -> Vec<PanelLine> {
        match kind {
            PanelKind::Weather => {
                let mut lines = vec![
                    PanelLine {
                        text: self.weather.temp_display(),
                        size_pt: 36,
                    },
                    PanelLine {
                        text: self.weather.condition().to_string(),
                        size_pt: 0,
                    },
                    PanelLine {
                        text: self.weather.humidity_line(),
                        size_pt: 0,
                    },
                    PanelLine {
                        text: self.weather.aqi_line(),
                        size_pt: 0,
                    },
                ];
                let status = self.weather.status_line();
                if !status.is_empty() {
                    lines.push(PanelLine {
                        text: status,
                        size_pt: 0,
                    });
                }
                lines
            }
            PanelKind::Calendar => self
                .calendar
                .events
                .iter()
                .take(3)
                .map(|ev| PanelLine {
                    text: ev.clone(),
                    size_pt: 0,
                })
                .collect(),
            PanelKind::Holidays => self
                .holidays
                .holidays
                .iter()
                .take(3)
                .map(|h| PanelLine {
                    text: h.clone(),
                    size_pt: 0,
                })
                .collect(),
        }
    }
}