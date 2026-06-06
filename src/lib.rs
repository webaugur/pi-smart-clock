//! Pi Smart Clock — shared core + platform-specific frontends.

#![cfg_attr(not(feature = "linux-full"), no_std)]

#[cfg(all(feature = "linux-full", feature = "pico-dvi"))]
compile_error!(
    "features `linux-full` and `pico-dvi` are mutually exclusive; \
     use `cargo run --features linux-full` on Linux or \
     `cargo pico` (alias for `cargo build --no-default-features --features pico-dvi --target thumbv6m-none-eabi`)"
);

#[cfg(not(feature = "linux-full"))]
#[macro_use]
extern crate alloc;

pub mod config;
pub mod prelude;
pub mod timing;
pub mod time_util;
pub mod clock_core;
pub mod drivers;
pub mod storage;
pub mod platform;
pub mod runtime;

#[cfg(feature = "linux-full")]
pub mod ota;

#[cfg(feature = "linux-full")]
pub mod web;

#[cfg(feature = "linux-full")]
pub mod chimes;

#[cfg(feature = "linux-full")]
pub mod clock;

pub mod layout;

#[cfg(feature = "linux-full")]
pub mod modules;

#[cfg(feature = "linux-full")]
pub mod panel;

#[cfg(feature = "linux-full")]
pub mod icons;