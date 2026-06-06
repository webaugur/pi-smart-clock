# Pi Smart Clock

Roman numeral smart clock — **Linux/SDL2 dev build** with autoscaling first, Pico DVI firmware on RP2040.

Detects your display orientation at startup: **portrait** screens use a 768×1280 logical layout; **landscape** screens use 1280×768. SDL letterboxes/scales to fit the window (95% of display size, resizable).

## Preview

<video controls playsinline width="100%" style="max-width: 480px;">
  <source src="https://github.com/webaugur/pi-smart-clock/raw/full-project/docs/images/default_clock.mp4" type="video/mp4">
  <a href="https://github.com/webaugur/pi-smart-clock/blob/full-project/docs/images/default_clock.mp4">Open demo video on GitHub</a>
</video>

## Documentation

| Guide | Description |
|-------|-------------|
| [docs/README.md](docs/README.md) | Index |
| [docs/CUSTOMIZATION.md](docs/CUSTOMIZATION.md) | Config files, SVG faces, fonts, modules, sounds |
| [docs/DRIVERS.md](docs/DRIVERS.md) | Hardware drivers (ESP8266, RTC, SD, sensors) |
| [docs/LINUX.md](docs/LINUX.md) | Linux / Unix install and persistence |
| [docs/EMBEDDED.md](docs/EMBEDDED.md) | Pico 1 & 2 firmware, hardware, SD card |
| [docs/SHARED_CODE.md](docs/SHARED_CODE.md) | Shared Rust architecture |

## Linux quickstart

### Rust toolchain (rustup)

Use **rustup** for `cargo` and `rustc`. Debian/Ubuntu packages (`apt install cargo rustc`) are too old for this project and conflict with embedded cross-builds.

```bash
# Install rustup (skip if you already have ~/.cargo/bin/cargo)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Update to the latest stable cargo + rustc
rustup update stable
rustup default stable

# Remove OS-packaged Rust so PATH does not pick up /usr/bin/cargo or /usr/bin/rustc
sudo apt remove -y cargo rustc
# Optional: purge config leftovers from the distro packages
# sudo apt purge -y cargo rustc

# Prefer rustup's binaries in every shell session
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
export PATH="$HOME/.cargo/bin:$PATH"

cargo --version
rustc --version
which cargo   # should print $HOME/.cargo/bin/cargo, not /usr/bin/cargo
```

### Build and run

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

**Customize:** copy `config/*.example` files — see [docs/CUSTOMIZATION.md](docs/CUSTOMIZATION.md).

**Chimes** — add WAVs under `sounds/` (see `sounds/README.txt`).

**Alarms** — `config/alarms.csv`; sound (WAV) + video (MP4 via ffmpeg on Linux).

---

## Embedded (Pico 1)

- **Pico 1 (RP2040)** + Pico DVI Sock (800×480), ESP8266, DS3231, SD, rotary encoder
- **Pico 2 (RP2350)** — planned; see [docs/EMBEDDED.md](docs/EMBEDDED.md)

```bash
./scripts/setup-embedded.sh          # once
./scripts/pico-build.sh --release
```

## Features

- Roman numeral clock with bounce second hand + night mode (amber)
- Weather / calendar / holiday panels
- Alarms, menu, time set, about screen
- NWS radar overlay during alerts (device)
- MQTT, OTA, voice (Pico build — in progress)

## License

See repository for license terms.