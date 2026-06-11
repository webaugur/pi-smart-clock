//! Pi Smart Clock — desktop clock for Unix targets (Debian Trixie and OpenIndiana 2025).
//! Single full desktop build using SDL2 + supporting crates.

pub mod config;
pub mod prelude;
pub mod timing;
pub mod time_util;
pub mod clock_core;
pub mod drivers;
pub mod storage;
pub mod platform;
pub mod runtime;

pub mod ota;
pub mod web;
pub mod chimes;
pub mod clock;

pub mod layout;
pub mod modules;
pub mod panel;
pub mod icons;