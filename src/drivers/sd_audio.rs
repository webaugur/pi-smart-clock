use crate::drivers::platform::Platform;

pub async fn play_wav_from_sd<P: Platform>(platform: &mut P, filename: &str) {
    #[cfg(feature = "linux-full")]
    println!("🎵 Playing from SD: {}", filename);
    #[cfg(not(feature = "linux-full"))]
    let _ = filename;
    // In real implementation: mount SD, read WAV, stream to I2S or PWM
    platform.play_raw_audio(filename).await;
}