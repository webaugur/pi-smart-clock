# Pi Smart Clock — Documentation

```
    DOCUMENT                          AUDIENCE / TOPIC
    --------                          ----------------
    CUSTOMIZATION.md                  Config files, assets, modules, sounds
    DRIVERS.md                        Hardware drivers (ESP8266, RTC, SD, …)
    LINUX.md                          Desktop / SDL2 build and deployment
    EMBEDDED.md                       Pico 1 & 2 firmware, hardware, SD card
    SHARED_CODE.md                    Rust architecture, features, shared modules
    TODO.md                           Detailed open-work tracker (IDs, files, status)
    ROADMAP.md                        Milestone phases and platform summary
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
| See what's left / track fixes | [TODO.md](TODO.md) · [ROADMAP.md](ROADMAP.md) |

## Build commands (cheat sheet)

```bash
# Linux (default)
cargo run --features linux-full

# Pico 1 firmware
./scripts/setup-embedded.sh          # once
./scripts/pico-build.sh --release
```

See platform-specific manuals for dependencies, PATH/rustup notes, and flashing.

```bash
# Audit inline TODO(ID) vs docs/TODO.md
./scripts/audit-todos.sh
```