# Pi Smart Clock

Roman numeral smart clock — **Linux/SDL2 dev build** first, Pico DVI firmware later.

## Linux quickstart (recommended)

```bash
# Debian/Ubuntu dependencies
sudo apt install -y libsdl2-dev libsdl2-ttf-dev libsdl2-mixer-dev fonts-dejavu-core

git clone https://github.com/webaugur/pi-smart-clock.git
cd pi-smart-clock
git checkout full-project

cargo run --features linux-full
# or: cargo run   (linux-full is default)
```

**Controls:** Esc quit · M menu · arrow keys / space = rotary encoder + button

**Optional sounds** — add short WAVs under `sounds/` (see `sounds/README.txt`):
`tick.wav`, `tock.wav`, `quarter.wav`, `half.wav`, `bell.wav`

**Alarms** — copy `config/alarms.csv.example` to `config/alarms.csv` and edit.

---

## Embedded hardware (Pico — later)

- Raspberry Pi Pico 1 (RP2040) + Pico DVI Sock (800×480)
- ESP8266 WiFi, DS3231 + SD, rotary encoder, I2S mic

```bash
cargo build --features pico-dvi --target thumbv6m-none-eabi --release
```

## Features
- Roman numeral clock with bounce second hand + night mode (amber)
- Weather / calendar / holiday panels (Linux stubs; live data on device)
- Alarms, menu, time set, about screen
- NWS radar overlay during alerts (device)
- MQTT, OTA, voice (Pico build)

## License
See repository for license terms.
