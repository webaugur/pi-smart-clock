//! Embedded storage: virtual paths map to the SD card volume at `/sd/`.
//! Access is via [`crate::drivers::sd_storage::SdStorage`] (I2C by default).

use crate::prelude::*;

pub fn resolve_logical_path(logical: &str) -> String {
    if logical.starts_with("cache/") {
        return format!("/sd/{logical}");
    }
    logical.to_string()
}