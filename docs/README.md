# Pi Smart Clock — Documentation

```
    DOCUMENT                          AUDIENCE / TOPIC
    --------                          ----------------
    CUSTOMIZATION.md                  Config files, assets, modules, sounds
    DRIVERS.md                        Hardware drivers (ESP8266, RTC, SD, …)
    LINUX.md                          Desktop / SDL2 build and deployment
    EMBEDDED.md                       Pico 1 & 2 firmware, hardware, SD card
    SHARED_CODE.md                    Rust architecture, features, shared modules
```

## Quick links

| I want to… | Read |
|------------|------|
| Change weather location, panels, alarms, WiFi | [CUSTOMIZATION.md](CUSTOMIZATION.md) |
| Wire or debug ESP8266, RTC, sensors, SD | [DRIVERS.md](DRIVERS.md) |
| Add or swap clock face SVGs, fonts, icons | [CUSTOMIZATION.md](CUSTOMIZATION.md#clock-faces) |
| Run on Linux (dev, kiosk, Raspberry Pi OS) | [LINUX.md](LINUX.md) |
| Build and flash Pico firmware | [EMBEDDED.md](EMBEDDED.md) |
| Understand `clock_core`, `Platform`, features | [SHARED_CODE.md](SHARED_CODE.md) |

## Build commands (cheat sheet)

```bash
# Linux (default)
cargo run --features linux-full

# Pico 1 firmware
./scripts/setup-embedded.sh          # once
./scripts/pico-build.sh --release
```

See platform-specific manuals for dependencies, PATH/rustup notes, and flashing.