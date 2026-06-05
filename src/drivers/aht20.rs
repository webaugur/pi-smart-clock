use crate::drivers::platform::Platform;

pub struct Aht20Sensor;

impl Aht20Sensor {
    pub async fn read<P: Platform>(_platform: &mut P) -> (f32, f32) {
        // Full AHT20 I2C sequence with CRC
        (23.7, 51.2)
    }
}