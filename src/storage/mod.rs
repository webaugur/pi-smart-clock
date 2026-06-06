//! Platform storage layout: XDG directories on Linux, `/sd/` on embedded.

#[cfg(feature = "linux-full")]
pub mod linux;

#[cfg(feature = "pico-dvi")]
pub mod embedded;

pub mod logical;