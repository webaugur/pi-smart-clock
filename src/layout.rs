//! Portrait display layout — 768×1280 (scaled 1.6× from 480×800 base).

pub const SCREEN_W: i32 = 768;
pub const SCREEN_H: i32 = 1280;

pub const WINDOW_W: u32 = 768;
pub const WINDOW_H: u32 = 1280;

/// Default TTF point size for UI text.
pub const FONT_SIZE: u16 = 32;

/// Roman clock face center.
pub const CLOCK_CX: i32 = 384;
pub const CLOCK_CY: i32 = 352;

pub const CLOCK_OUTER_R: i32 = 304;
pub const CLOCK_INNER_R: i32 = 280;
pub const TICK_OUTER_R: i32 = 304;
pub const TICK_INNER_R: i32 = 272;
pub const HAND_LENGTH: i32 = 264;
pub const ROMAN_RADIUS: i32 = 248;

/// Center panel — weather / alarm video.
pub const CENTER_X: i32 = 171;
pub const CENTER_Y: i32 = 480;
pub const CENTER_W: u32 = 426;
pub const CENTER_H: u32 = 256;

/// Bottom third — calendar & holidays.
pub const BOTTOM_Y: i32 = 853;
pub const BOTTOM_H: i32 = 411;

pub const CAL_X: i32 = 0;
pub const CAL_Y: i32 = BOTTOM_Y;
pub const CAL_W: i32 = 384;
pub const CAL_H: i32 = BOTTOM_H;

pub const HOL_X: i32 = 384;
pub const HOL_Y: i32 = BOTTOM_Y;
pub const HOL_W: i32 = 384;
pub const HOL_H: i32 = BOTTOM_H;

/// Status bar along the bottom edge.
pub const STATUS_Y: i32 = 1256;