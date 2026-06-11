//! Runtime layout — fixed 4:3 vertical (3:4 width:height) logical coords, scaled to display height.

#[cfg(feature = "full")]
use std::sync::OnceLock;

#[cfg(feature = "full")]
static ACTIVE: OnceLock<Layout> = OnceLock::new();

#[cfg(not(feature = "full"))]
static ACTIVE_LAYOUT: Layout = Layout::dvi_vga();

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub orientation: Orientation,
    pub screen_w: i32,
    pub screen_h: i32,
    pub font_size: u16,
    pub clock_cx: i32,
    pub clock_cy: i32,
    pub clock_outer_r: i32,
    pub clock_inner_r: i32,
    pub tick_outer_r: i32,
    pub tick_inner_r: i32,
    pub hand_length: i32,
    pub roman_radius: i32,
    pub center_x: i32,
    pub center_y: i32,
    pub center_w: u32,
    pub center_h: u32,
    pub bottom_y: i32,
    pub bottom_h: i32,
    // New upper row (second layer) above the traditional bottom row.
    // Same visual style and module format. Allocated by shrinking/splitting the old bottom area.
    pub upper_y: i32,
    pub upper_h: i32,
    pub panel_w: i32,
    pub weather_x: i32,
    pub weather_y: i32,
    pub weather_w: i32,
    pub weather_h: i32,
    pub cal_x: i32,
    pub cal_y: i32,
    pub cal_w: i32,
    pub cal_h: i32,
    pub hol_x: i32,
    pub hol_y: i32,
    pub hol_w: i32,
    pub hol_h: i32,
    pub status_y: i32,
    pub bottom_title_pt: u8,
    pub bottom_body_pt: u8,
    pub bottom_line_gap: i32,
}

impl Layout {
    /// 4:3 vertical (portrait): 960×1280 logical pixels (width:height = 3:4).
    pub const fn portrait() -> Self {
        let panel_w = 320;
        // Allocate a second "upper" row (new layer) by splitting the previous bottom area.
        // Upper row sits directly above the traditional bottom row. Same module format.
        let band_h = 150;
        let upper_y = 920;
        let bottom_y = upper_y + band_h;
        Self {
            orientation: Orientation::Portrait,
            screen_w: 960,
            screen_h: 1280,
            font_size: 32,
            clock_cx: 480,
            clock_cy: 330,
            clock_outer_r: 260,
            clock_inner_r: 238,
            tick_outer_r: 260,
            tick_inner_r: 230,
            hand_length: 215,
            roman_radius: 200,
            center_x: 267,
            center_y: 610,
            center_w: 426,
            center_h: 240,
            bottom_y,
            bottom_h: band_h,
            upper_y,
            upper_h: band_h,
            panel_w,
            weather_x: 0,
            weather_y: bottom_y,
            weather_w: panel_w,
            weather_h: band_h,
            cal_x: panel_w,
            cal_y: bottom_y,
            cal_w: panel_w,
            cal_h: band_h,
            hol_x: panel_w * 2,
            hol_y: bottom_y,
            hol_w: panel_w,
            hol_h: band_h,
            status_y: 1256,
            bottom_title_pt: 30,
            bottom_body_pt: 26,
            bottom_line_gap: 42,
        }
    }

    /// 640×480 landscape — matches Pico DVI VGA timing (see `pico_dvi_rs::DISPLAY_*`).
    pub const fn dvi_vga() -> Self {
        let panel_w = 213;
        let bottom_h = 120;
        let bottom_y = 340;
        Self {
            orientation: Orientation::Landscape,
            screen_w: 640,
            screen_h: 480,
            font_size: 16,
            clock_cx: 320,
            clock_cy: 200,
            clock_outer_r: 150,
            clock_inner_r: 138,
            tick_outer_r: 150,
            tick_inner_r: 132,
            hand_length: 120,
            roman_radius: 112,
            center_x: 160,
            center_y: 280,
            center_w: 320,
            center_h: 120,
            bottom_y,
            bottom_h,
            upper_y: bottom_y, // no separate upper band in this legacy DVI layout; reuse bottom for compatibility
            upper_h: 0,
            panel_w,
            weather_x: 0,
            weather_y: bottom_y,
            weather_w: panel_w,
            weather_h: bottom_h,
            cal_x: panel_w,
            cal_y: bottom_y,
            cal_w: panel_w,
            cal_h: bottom_h,
            hol_x: panel_w * 2,
            hol_y: bottom_y,
            hol_w: panel_w,
            hol_h: bottom_h,
            status_y: 468,
            bottom_title_pt: 18,
            bottom_body_pt: 16,
            bottom_line_gap: 24,
        }
    }

    pub fn landscape() -> Self {
        Self {
            orientation: Orientation::Landscape,
            screen_w: 1280,
            screen_h: 768,
            font_size: 32,
            clock_cx: 640,
            clock_cy: 320,
            clock_outer_r: 304,
            clock_inner_r: 280,
            tick_outer_r: 304,
            tick_inner_r: 272,
            hand_length: 264,
            roman_radius: 248,
            center_x: 427,
            center_y: 224,
            center_w: 426,
            center_h: 256,
            bottom_y: 512,
            bottom_h: 256,
            upper_y: 512,
            upper_h: 0, // upper row geometry added for the new second layer (primarily used in portrait)
            cal_x: 0,
            cal_y: 512,
            cal_w: 384,
            cal_h: 256,
            hol_x: 896,
            hol_y: 512,
            hol_w: 384,
            hol_h: 256,
            status_y: 752,
            panel_w: 384,
            weather_x: 0,
            weather_y: 512,
            weather_w: 384,
            weather_h: 256,
            bottom_title_pt: 30,
            bottom_body_pt: 26,
            bottom_line_gap: 42,
        }
    }

    /// Register the fixed 4:3 vertical layout (always portrait).
    pub fn init(_display_w: u32, _display_h: u32) -> &'static Layout {
        #[cfg(feature = "full")]
        {
            let layout = Self::portrait();
            let _ = ACTIVE.set(layout);
            return ACTIVE.get().unwrap();
        }
        #[cfg(not(feature = "full"))]
        {
            &ACTIVE_LAYOUT
        }
    }

    /// Window size: ~95% of display height; width follows layout aspect ratio.
    #[cfg(feature = "full")]
    pub fn window_size(_display_w: u32, display_h: u32) -> (u32, u32) {
        let h = ((display_h as f32) * 0.95).round() as u32;
        Self::size_for_height(h)
    }

    #[cfg(feature = "full")]
    const MIN_WINDOW_H: u32 = 240;

    /// Derive width from a target height while preserving layout aspect ratio.
    #[cfg(feature = "full")]
    pub fn size_for_height(h: u32) -> (u32, u32) {
        let layout = l();
        let h = h.max(Self::MIN_WINDOW_H);
        let aspect = layout.screen_w as f64 / layout.screen_h as f64;
        let min_w = (Self::MIN_WINDOW_H as f64 * aspect).round() as u32;
        let w = (h as f64 * aspect).round() as u32;
        (w.max(min_w), h)
    }

    /// Snap arbitrary dimensions to the active layout aspect ratio.
    #[cfg(feature = "full")]
    pub fn snap_window_size(w: u32, h: u32) -> (u32, u32) {
        let layout = l();
        let aspect = layout.screen_w as f64 / layout.screen_h as f64;
        let current = w as f64 / h.max(1) as f64;
        let min_w = (Self::MIN_WINDOW_H as f64 * aspect).round() as u32;

        if (current - aspect).abs() < 0.005 {
            return (w.max(min_w), h.max(Self::MIN_WINDOW_H));
        }

        if current > aspect {
            let w = (h as f64 * aspect).round() as u32;
            (w.max(min_w), h.max(Self::MIN_WINDOW_H))
        } else {
            let h = (w as f64 / aspect).round() as u32;
            (w.max(min_w), h.max(Self::MIN_WINDOW_H))
        }
    }

    /// Smallest allowed window size, preserving layout aspect ratio.
    #[cfg(feature = "full")]
    pub fn minimum_window_size() -> (u32, u32) {
        Self::snap_window_size(1, Self::MIN_WINDOW_H)
    }

    /// Bottom panel slot rect (Linux bottom panel bar).
    #[cfg(feature = "full")]
    pub fn bottom_slot(&self, slot: crate::modules::slot::BottomSlot) -> (i32, i32, i32, i32) {
        let x = self.panel_w * slot.index() as i32;
        (x, self.bottom_y, self.panel_w, self.bottom_h)
    }

    /// Upper row slot rect (new second layer above the traditional bottom row).
    /// Uses the exact same visual style and module format as bottom slots.
    #[cfg(feature = "full")]
    pub fn upper_slot(&self, slot: crate::modules::slot::BottomSlot) -> (i32, i32, i32, i32) {
        let x = self.panel_w * slot.index() as i32;
        (x, self.upper_y, self.panel_w, self.upper_h)
    }
}

/// Active layout (call `Layout::init` from `main` first on Linux).
pub fn l() -> &'static Layout {
    #[cfg(feature = "full")]
    {
        return ACTIVE.get().expect("layout::Layout::init not called");
    }
    #[cfg(not(feature = "full"))]
    {
        &ACTIVE_LAYOUT
    }
}