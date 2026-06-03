#![cfg_attr(feature = "pico-dvi", no_std)]
#![cfg_attr(feature = "pico-dvi", no_main)]

mod config;
mod core;
mod drivers;
mod platform;
mod web;
mod ota;

use chrono::Local;

#[cfg(feature = "linux-full")]
#[tokio::main]
async fn main() -> Result<(), String> {
    println!("\n\u{1F680} Smart Clock v0.1.0 - Linux Mode");
    let mut platform = platform::linux::LinuxPlatform::new()?;
    run_app(&mut platform).await
}

#[cfg(feature = "pico-dvi")]
#[embassy_executor::main]
async fn main() -> ! {
    println!("\n\u{1F680} Smart Clock v0.1.0 - Pico DVI Mode");
    let mut platform = platform::rp2040::PicoDviPlatform::new();
    run_app(&mut platform).await;
    loop {}
}

async fn run_app<P: drivers::platform::Platform>(platform: &mut P) {
    // Boot screen
    core::boot_screen::show(platform).await;

    // Initialize all systems
    platform.init().await.expect("Platform init failed");

    // DS3231 time sync
    drivers::ds3231::DS3231::synchronize(platform).await;

    // Start main systems
    let mut scheduler = core::update_scheduler::UpdateScheduler::new();
    let mut alert_manager = core::alerts::AlertManager::new();
    let mut sensor = core::sensors::Aht20Sensor::new();
    let mut alarm_manager = core::alarm::AlarmManager::new();
    let mut mqtt = drivers::mqtt::MqttClient::new();
    let mut ota = ota::updater::OtaUpdater::new();

    // MQTT setup (LAN)
    mqtt.connect(platform, config::MQTT_BROKER, config::MQTT_PORT, config::MQTT_USER, config::MQTT_PASS).await;
    mqtt.subscribe(platform, "smartclock/#").await;

    // Main loop
    loop {
        // Update scheduler (smart timing)
        scheduler.tick(platform, &alert_manager).await;

        // Read sensors
        sensor.read(platform).await;

        // Check alerts
        alert_manager.check_nws_alerts(platform).await;

        // Draw everything
        core::clock::update(platform).await;
        core::status_bar::draw(platform, sensor.temp_c, env!("GIT_HASH")).await;
        core::weather::draw(platform, &sensor).await;

        // Process alarms
        alarm_manager.check(platform, Local::now()).await;

        // Process MQTT commands
        mqtt.process_incoming(platform).await;

        // OTA check (if enabled)
        if ota.enabled {
            ota.check_web_update(platform).await;
        }

        platform.present();
        platform.delay_ms(16).await;
    }
}