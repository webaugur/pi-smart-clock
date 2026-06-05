//! Portrait display layout (480×800), matching the physical panel orientation.

pub const SCREEN_W: i32 = 480;
pub const SCREEN_H: i32 = 800;

pub const WINDOW_W: u32 = 480;
pub const WINDOW_H: u32 = 800;

/// Roman clock face center.
pub const CLOCK_CX: i32 = 240;
pub const CLOCK_CY: i32 = 220;

/// Center panel — weather / alarm video.
pub const CENTER_X: i32 = 107;
pub const CENTER_Y: i32 = 300;
pub const CENTER_W: u32 = 266;
pub const CENTER_H: u32 = 160;

/// Bottom third — calendar & holidays.
pub const BOTTOM_Y: i32 = 533;
pub const BOTTOM_H: i32 = 257;

pub const CAL_X: i32 = 0;
pub const CAL_Y: i32 = BOTTOM_Y;
pub const CAL_W: i32 = 240;
pub const CAL_H: i32 = BOTTOM_H;

pub const HOL_X: i32 = 240;
pub const HOL_Y: i32 = BOTTOM_Y;
pub const HOL_W: i32 = 240;
pub const HOL_H: i32 = BOTTOM_H;

/// Status bar along the bottom edge.
pub const STATUS_Y: i32 = 785;