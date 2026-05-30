# Smart Clock - Raspberry Pi Pico 1 + DVI Sock

**Complete embedded smart clock with Roman numerals, weather radar, Amber/Silver alerts, voice, MQTT, OTA, and more.**

## Features
- Roman numeral clock with mechanical bounce + candle-flicker night mode
- DS3231 RTC + temperature compensation
- AHT20 temperature & humidity
- NWS weather radar (only during alerts)
- Amber/Silver alert photo display (bottom center panel)
- Rotary encoder + push button navigation
- SD card (CSV alarms, sounds, backups, photos)
- Full Web UI (mobile-friendly, dark mode, config page)
- MQTT (LAN + Home Assistant discovery + subscriptions + LWT)
- OTA updates (Web + MQTT, with safe rollback + USB fallback)
- Voice commands + periodic announcements (every 15 min during alerts)
- Energy monitoring for Alexa smart outlets
- Multi-language support (English, Spanish, German, French)
- Git commit hash display

## Hardware
- Raspberry Pi Pico 1 (RP2040)
- Pico DVI Sock (800x480 @ 60Hz)
- ESP8266 WiFi module
- DS3231 + microSD combo board
- Rotary encoder + center SPST button
- I2S microphone(s)

## Build Instructions
```bash
# Linux development version
cargo run --features linux-full

# Pico embedded version
cargo build --features pico-dvi --target thumbv6m-none-eabi --release
```