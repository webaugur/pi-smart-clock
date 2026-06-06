# Pi Smart Clock

Roman numeral smart clock — **Linux/SDL2 dev build** with autoscaling first, Pico DVI firmware later.

Detects your display orientation at startup: **portrait** screens use a 768×1280 logical layout; **landscape** screens use 1280×768. SDL letterboxes/scales to fit the window (95% of display size, resizable).

## Linux quickstart (recommended)

```bash
# Debian/Ubuntu dependencies
sudo apt install -y libsdl2-dev libsdl2-ttf-dev libsdl2-mixer-dev fonts-dejavu-core ffmpeg

git clone https://github.com/webaugur/pi-smart-clock.git
cd pi-smart-clock
git checkout full-project

cargo run --features linux-full
# or: cargo run   (linux-full is default)
```

**Controls:** Esc quit · M menu · arrow keys / space = rotary encoder + button

**Chimes** — add WAVs under `sounds/` (see `sounds/README.txt`):
`tick.wav`, `tock.wav`, `quarter.wav`, `half.wav`, `bell.wav`

**Alarms** — copy `config/alarms.csv.example` to `config/alarms.csv`. Set `sound_file` (WAV) and `video_file` (MP4 under `videos/`). Alarm video plays in the center panel via ffmpeg; sound loops until dismissed (button/space).

**Test alarm:** set hour/minute one minute ahead, `enabled,true`, valid paths to your media files.

---

## Embedded hardware (Pico)

- **Pico 1 (RP2040)** — supported now: Pico DVI Sock (800×480), ESP8266 WiFi, DS3231 + SD, rotary encoder
- **Pico 2 (RP2350)** — planned (different Rust target / chip; see manual)

Full wiring, SD card config, flashing, and Pico 1 vs Pico 2 differences:
**[docs/CONFIGURATION_MANUAL.md — Chapter 10](docs/CONFIGURATION_MANUAL.md#chapter-10--pi-pico-1-and-pi-pico-2--installation-and-configuration)**

```bash
# One-time: ./scripts/setup-embedded.sh
./scripts/pico-build.sh              # debug
./scripts/pico-build.sh --release    # release
```

## Features
- Roman numeral clock with bounce second hand + night mode (amber)
- Weather / calendar / holiday panels (Linux stubs; live data on device)
- Alarms, menu, time set, about screen
- NWS radar overlay during alerts (device)
- MQTT, OTA, voice (Pico build)

## License
See repository for license terms.
