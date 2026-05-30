# Smart Clock

**Raspberry Pi Pico 1 + DVI Sock + ESP8266**

Full-featured smart clock with Roman numerals, weather radar, Amber/Silver alerts, voice control, MQTT, OTA updates, and more.

## Features
- Roman numeral clock with mechanical second hand + candle flicker night mode
- DS3231 RTC + AHT20 sensor
- Weather radar + NWS SAME code alerts
- Amber/Silver alert photo display
- Voice commands + announcements
- Rotary encoder menu + time setting
- Web UI (mobile friendly, dark mode)
- MQTT (LAN + Home Assistant discovery)
- OTA updates (Web + MQTT, off by default)
- SD card support (alarms, sounds, backups)
- Energy monitoring
- Multi-language

## Hardware
- Raspberry Pi Pico 1 (RP2040)
- Pico DVI Sock
- ESP8266 WiFi
- DS3231 RTC + SD card module
- Rotary encoder with push button
- I2S microphone(s)

## Build
```bash
# Linux version
cargo run --features linux-full

# Pico DVI version
cargo build --features pico-dvi --target thumbv6m-none-eabi --release
```