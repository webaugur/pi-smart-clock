//! Runtime layout profiles — portrait or landscape logical coords, SDL-scaled to the display.

use std::sync::OnceLock;

static ACTIVE: OnceLock<Layout> = OnceLock::new();

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
    pub cal_x: i32,
    pub cal_y: i32,
    pub cal_w: i32,
    pub cal_h: i32,
    pub hol_x: i32,
    pub hol_y: i32,
    pub hol_w: i32,
    pub hol_h: i32,
    pub status_y: i32,
}

impl Layout {
    pub fn portrait() -> Self {
        Self {
            orientation: Orientation::Portrait,
            screen_w: 768,
            screen_h: 1280,
            font_size: 32,
            clock_cx: 384,
            clock_cy: 352,
            clock_outer_r: 304,
            clock_inner_r: 280,
            tick_outer_r: 304,
            tick_inner_r: 272,
            hand_length: 264,
            roman_radius: 248,
            center_x: 171,
            center_y: 480,
            center_w: 426,
            center_h: 256,
            bottom_y: 853,
            bottom_h: 411,
            cal_x: 0,
            cal_y: 853,
            cal_w: 384,
            cal_h: 411,
            hol_x: 384,
            hol_y: 853,
            hol_w: 384,
            hol_h: 411,
            status_y: 1256,
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
            cal_x: 0,
            cal_y: 512,
            cal_w: 384,
            cal_h: 256,
            hol_x: 896,
            hol_y: 512,
            hol_w: 384,
            hol_h: 256,
            status_y: 752,
        }
    }

    /// Pick layout from display aspect ratio and register as the active profile.
    pub fn init(display_w: u32, display_h: u32) -> &'static Layout {
        let layout = if display_w >= display_h {
            Self::landscape()
        } else {
            Self::portrait()
        };
        let _ = ACTIVE.set(layout);
        ACTIVE.get().unwrap()
    }

    /// Window size: ~95% of display, preserving orientation.
    pub fn window_size(display_w: u32, display_h: u32) -> (u32, u32) {
        let w = ((display_w as f32) * 0.95).round() as u32;
        let h = ((display_h as f32) * 0.95).round() as u32;
        (w.max(320), h.max(240))
    }
}

/// Active layout (call `Layout::init` from `main` first).
pub fn l() -> &'static Layout {
    ACTIVE.get().expect("layout::Layout::init not called")
}