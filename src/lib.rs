//! Pi Smart Clock — shared core + platform-specific frontends.

pub mod config;
pub mod core;
pub mod drivers;
pub mod ota;
pub mod platform;
pub mod runtime;
pub mod web;

#[cfg(feature = "linux-full")]
pub mod chimes;

#[cfg(feature = "linux-full")]
pub mod clock;

#[cfg(feature = "linux-full")]
pub mod layout;

#[cfg(feature = "linux-full")]
pub mod modules;

#[cfg(feature = "linux-full")]
pub mod panel;

#[cfg(feature = "linux-full")]
pub mod icons;
