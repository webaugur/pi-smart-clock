use crate::drivers::platform::Platform;

pub struct EnvSensor {
    pub temp_c: f32,
    pub humidity: f32,
}

impl EnvSensor {
    pub fn new() -> Self {
        Self { temp_c: 22.5, humidity: 48.0 }
    }

    pub async fn read<P: Platform>(&mut self, platform: &mut P) {
        // TODO: Read from AHT20 or DS3231
        self.temp_c = 23.4;
        self.humidity = 47.0;
    }
}