use crate::drivers::platform::Platform;

pub struct DS3231;

impl DS3231 {
    pub const ADDR: u8 = 0x68;

    pub async fn synchronize<P: Platform>(platform: &mut P) {
        // Read time from DS3231 and set system time
        let _ = ();
    }

    pub async fn set_time<P: Platform>(platform: &mut P, hour: u32, minute: u32) {
        // Write time to DS3231
    }
}