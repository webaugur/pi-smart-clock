use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::drivers::platform::Platform;
use crate::layout::Layout;
use crate::modules::config::BottomSlotConfig;
use crate::modules::module_id::ModuleId;
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
        // Tick the new upper-row modules (Lunar, Zodiac, Pentagram) – they live in the registry.
        self.registry.tick(ModuleId::Lunar, alerts_active);
        self.registry.tick(ModuleId::Zodiac, alerts_active);
        self.registry.tick(ModuleId::Pentagram, alerts_active);
    }

    pub fn draw_backgrounds(&mut self, canvas: &mut Canvas<Window>, layout: &Layout) {
        // Original bottom row (3 slots)
        for slot in BottomSlot::ALL {
            let module = self.slots.module_for(slot);
            let (x, y, w, h) = layout.bottom_slot(slot);
            self.registry
                .draw_background(module, canvas, x, y, w, h);
        }

        // New upper row (second layer) – the 3 new astro modules (Lunar / Zodiac / Pentagram)
        // using the exact same module format and layout helpers.
        let upper_modules = [ModuleId::Lunar, ModuleId::Zodiac, ModuleId::Pentagram];
        for (i, &mid) in upper_modules.iter().enumerate() {
            // Reuse BottomSlot indexing for x position (0,1,2)
            let slot = match i {
                0 => BottomSlot::Left,
                1 => BottomSlot::Mid,
                _ => BottomSlot::Right,
            };
            let (x, y, w, h) = layout.upper_slot(slot);
            self.registry.draw_background(mid, canvas, x, y, w, h);
        }
    }

    pub async fn draw_content<P: Platform>(&mut self, platform: &mut P, layout: &Layout) {
        let pad = 10;

        // Bottom row content (original 3)
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

        // Upper row content (the 3 new modules)
        let upper_body_y = layout.upper_y + 40;
        let upper_modules = [ModuleId::Lunar, ModuleId::Zodiac, ModuleId::Pentagram];
        for (i, &mid) in upper_modules.iter().enumerate() {
            let slot = match i {
                0 => BottomSlot::Left,
                1 => BottomSlot::Mid,
                _ => BottomSlot::Right,
            };
            let (x, _, _, _) = layout.upper_slot(slot);
            let (title, title_color) = self.registry.title(mid);
            platform
                .draw_text(
                    &title,
                    x + pad,
                    layout.upper_y + 6,
                    layout.bottom_title_pt, // reuse same title size for visual consistency
                    title_color,
                )
                .await;

            for (j, line) in self.registry.lines(mid).into_iter().enumerate() {
                let size = if line.size_pt == 0 {
                    layout.bottom_body_pt
                } else {
                    line.size_pt
                };
                platform
                    .draw_text(
                        &line.text,
                        x + pad,
                        upper_body_y + (j as i32) * layout.bottom_line_gap,
                        size,
                        0xCCCCCC,
                    )
                    .await;
            }
        }
    }
}