use crate::drivers::platform::Platform;

pub async fn speak<P: Platform>(platform: &mut P, message: &str) {
    println!("\ud83d\udde3️ Speaking: {}", message);
    // In real implementation this would play pre-recorded or generated WAV from SD card
    // For now we just log it
    platform.draw_text(&format!("\ud83d\udde3️ {}", message), 50, 50, 18, 0x00FFAA);
}