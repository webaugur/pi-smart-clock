// Full main.rs with all features integrated
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

mod config;
mod core;
mod drivers;
mod platform;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("\n=== Smart Clock v0.2.0 ===");
    println!("Hardware: Pico 1 + DVI Sock + ESP8266");
    println!("Features: Radar, Alerts, Voice, MQTT, OTA, Rotary\n");

    let mut platform = platform::rp2040::PicoDviPlatform::new();
    platform.init().await.expect("Platform init failed");

    core::clock::start(&mut platform).await;
    core::alerts::start(&mut platform).await;
    core::sensors::start(&mut platform).await;

    loop {
        core::update_scheduler::tick(&mut platform).await;
        Timer::after_millis(50).await;
    }
}