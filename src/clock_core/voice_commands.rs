use crate::drivers::platform::Platform;

pub async fn process<P: Platform>(platform: &mut P, text: &str) {
    let cmd = text.to_lowercase();
    if cmd.contains("set alarm") {
        platform.speak("Alarm set").await;
    } else if cmd.contains("weather") {
        platform.show_weather().await;
    }
}