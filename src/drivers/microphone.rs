use crate::drivers::platform::Platform;

pub struct VoiceInput {
    pub wake_word_detected: bool,
}

impl VoiceInput {
    pub fn new() -> Self {
        Self { wake_word_detected: false }
    }

    pub async fn listen<P: Platform>(&mut self, platform: &mut P) {
        let buffer = platform.read_i2s_samples(512).await;
        if buffer.contains_hotword("HEY CLOCK") || buffer.contains_hotword("ALARM") {
            self.wake_word_detected = true;
            println!("🎤 Wake word detected!");
        }
    }
}