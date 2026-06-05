use crate::drivers::platform::Platform;

pub const DEFAULT_ALARM_SOUNDS: [&str; 4] = [
    "sounds/cuckoo.wav",
    "sounds/bell.wav",
    "sounds/chime.wav",
    "sounds/alarm.wav",
];

pub async fn select_alarm_sound<P: Platform>(platform: &mut P, index: usize) -> String {
    let sound = DEFAULT_ALARM_SOUNDS
        .get(index % DEFAULT_ALARM_SOUNDS.len())
        .copied()
        .unwrap_or("sounds/cuckoo.wav");

    platform.play_sound(sound, 0.8).await;
    sound.to_string()
}

pub async fn preview_sound<P: Platform>(platform: &mut P, path: &str) {
    platform.play_sound(path, 1.0).await;
}