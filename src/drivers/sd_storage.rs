//! SD card filesystem access for embedded builds.
//!
//! Default bus: **I2C** (slower, works with shared sensor bus wiring).
//! Planned faster modes: SPI, SDIO (see [`StorageBusMode`]).

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StorageBusMode {
    /// Current default — SD module on I2C GPIO expander / shared bus.
    #[default]
    I2c,
    /// Planned: dedicated SPI SD slot.
    Spi,
    /// Planned: SDIO high-speed mode.
    Sdio,
}

impl StorageBusMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::I2c => "i2c",
            Self::Spi => "spi",
            Self::Sdio => "sdio",
        }
    }
}

/// Block storage backed by removable SD media.
pub struct SdStorage {
    mode: StorageBusMode,
    mounted: bool,
}

impl SdStorage {
    pub fn new(mode: StorageBusMode) -> Self {
        Self {
            mode,
            mounted: false,
        }
    }

    pub fn mode(&self) -> StorageBusMode {
        self.mode
    }

    pub fn is_mounted(&self) -> bool {
        self.mounted
    }

    /// Mount the volume at `/sd` over the configured bus.
    pub fn mount(&mut self) -> Result<(), String> {
        // TODO: probe card over I2C, FAT mount, publish /sd root.
        self.mounted = true;
        Ok(())
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, String> {
        if !self.mounted {
            return Err(String::from("SD card not mounted"));
        }
        if !path.starts_with("/sd/") {
            return Err(format!("embedded paths must start with /sd/: {path}"));
        }
        // TODO: I2C block read → FAT file open → read_all.
        Err(format!(
            "SD read not yet implemented ({}, {} mode)",
            path,
            self.mode.as_str()
        ))
    }

    pub fn write_file(&mut self, path: &str, data: &[u8]) -> Result<(), String> {
        if !self.mounted {
            return Err(String::from("SD card not mounted"));
        }
        if !path.starts_with("/sd/") {
            return Err(format!("embedded paths must start with /sd/: {path}"));
        }
        let _ = data;
        // TODO: I2C block write → FAT create/truncate → write_all.
        Err(format!(
            "SD write not yet implemented ({}, {} mode)",
            path,
            self.mode.as_str()
        ))
    }

    pub fn copy_file(&mut self, from: &str, to: &str) -> Result<(), String> {
        let data = self.read_file(from)?;
        self.write_file(to, &data)
    }
}