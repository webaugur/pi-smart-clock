use crate::drivers::platform::Platform;

pub struct VoiceInput {
    pub wake_word_detected: bool,
    pub last_energy: u32,
}

impl VoiceInput {
    pub fn new() -> Self {
        Self {
            wake_word_detected: false,
            last_energy: 0,
        }
    }

    pub async fn listen<P: Platform>(&mut self, platform: &mut P) {
        let buffer = platform.read_i2s_samples(512).await;
        let energy: u32 = buffer.iter().map(|s| s.unsigned_abs() as u32).sum();
        self.last_energy = energy;
        if energy > 50_000 {
            self.wake_word_detected = true;
        }
    }
}