use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::drivers::platform::Platform;
use crate::layout::Layout;
use crate::modules::config::BottomSlotConfig;
use crate::modules::registry::ModuleRegistry;
use crate::modules::slot::BottomSlot;

pub struct BottomPanelBar {
    pub slots: BottomSlotConfig,
    registry: ModuleRegistry,
}

impl BottomPanelBar {
    pub fn new() -> Self {
        Self::with_slots(crate::modules::config::load_bottom_slots())
    }

    pub fn with_slots(slots: BottomSlotConfig) -> Self {
        Self {
            slots,
            registry: ModuleRegistry::new(),
        }
    }

    pub fn weather_mut(&mut self) -> &mut crate::modules::weather::WeatherPanel {
        self.registry.weather_mut()
    }

    pub fn tick(&mut self, alerts_active: bool) {
        for slot in BottomSlot::ALL {
            let module = self.slots.module_for(slot);
            self.registry.tick(module, alerts_active);
        }
    }

    pub fn draw_backgrounds(&mut self, canvas: &mut Canvas<Window>, layout: &Layout) {
        for slot in BottomSlot::ALL {
            let module = self.slots.module_for(slot);
            let (x, y, w, h) = layout.bottom_slot(slot);
            self.registry
                .draw_background(module, canvas, x, y, w, h);
        }
    }

    pub async fn draw_content<P: Platform>(&mut self, platform: &mut P, layout: &Layout) {
        let pad = 10;
        let body_y = layout.bottom_y + 40;

        for slot in BottomSlot::ALL {
            let module = self.slots.module_for(slot);
            let (x, _, _, _) = layout.bottom_slot(slot);
            let (title, title_color) = self.registry.title(module);
            platform
                .draw_text(
                    &title,
                    x + pad,
                    layout.bottom_y + 6,
                    layout.bottom_title_pt,
                    title_color,
                )
                .await;

            for (i, line) in self.registry.lines(module).into_iter().enumerate() {
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
}