use crate::drivers::platform::Platform;

pub struct VoiceFeedback;

impl VoiceFeedback {
    pub async fn speak<P: Platform>(platform: &mut P, message: &str) {
        #[cfg(feature = "linux-full")]
        println!("🔊 Speaking: {}", message);
        
        // TODO: Map message to WAV file on SD card
        // Example: "alarm set" -> "voice/alarm_set.wav"
        let wav_file = match message.to_lowercase().as_str() {
            m if m.contains("alarm") => "voice/alarm_set.wav",
            m if m.contains("good morning") => "voice/good_morning.wav",
            _ => "voice/acknowledge.wav",
        };
        
        platform.play_sound(wav_file, 0.9).await;
    }
}