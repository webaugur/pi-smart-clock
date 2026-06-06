# Linux / Unix Guide

SDL2 desktop build for development, kiosk images, and **Raspberry Pi OS Trixie** (Debian 13). Default Cargo feature: `linux-full`.

We target **Debian Trixie** for native Linux builds — same base as current Pi OS. Older releases (Bookworm, Ubuntu LTS, etc.) may work but are not the reference environment.

For config files and assets see [CUSTOMIZATION.md](CUSTOMIZATION.md).  
For shared Rust layout see [SHARED_CODE.md](SHARED_CODE.md).

---

## Quick start

On **Debian 13 Trixie** or **Raspberry Pi OS Trixie**:

```bash
git clone https://github.com/webaugur/pi-smart-clock.git
cd pi-smart-clock
git checkout full-project

./scripts/linux-deps.sh
./scripts/linux-build.sh
cargo run --features linux-full
# or: cargo run   (linux-full is default)
```

### Docker / CI

Reproducible Trixie build (no local SDL packages needed):

```bash
docker build -t pi-smart-clock .
```

GitHub Actions uses `debian:trixie-slim` — see `.github/workflows/linux-trixie.yml`.

**Controls**

| Input | Action |
|-------|--------|
| Esc | Quit |
| M | Menu |
| Arrow keys | Rotary encoder |
| Space | Push button |

---

## Display and layout

- Detects orientation at startup: **portrait** → 768×1280 logical; **landscape** → 1280×768.
- SDL window scales to ~95% of display size, letterboxed, resizable.
- Bottom panel slots and clock face come from config — [CUSTOMIZATION.md](CUSTOMIZATION.md).

---

## Persistence

### Config search order

For each `*.conf` name:

```
1. ~/.config/pi-smart-clock/<name>.conf
2. config/<name>.conf              (repo / cwd)
3. config/<name>.conf.example
```

Missing files → built-in defaults + stderr notice.

### XDG layout

```
~/.config/pi-smart-clock/     Settings (weather, panels, faces, esp8266)
~/.local/share/pi-smart-clock/ Data (alarms.csv, alert photos)
~/.local/state/pi-smart-clock/ Backups (alarm CSV history)
~/.cache/pi-smart-clock/       Cache (weather JSON)
```

Virtual `/sd/...` and `cache/...` paths in code map to these directories via `src/storage/linux.rs`.

### Deployment and branding

Ship defaults in repo `config/` for factory/kiosk images. User saves override `~/.config/pi-smart-clock/` only.

**Recovery** (broken user config):

```bash
rm -rf ~/.config/pi-smart-clock
```

Does not remove alarms under `~/.local/share/` or `~/.local/state/`. Clear weather cache separately:

```bash
rm -rf ~/.cache/pi-smart-clock
```

---

## Alarms and media (Linux-specific)

- Copy `config/alarms.csv.example` → `config/alarms.csv` or edit persisted copy under `~/.local/share/`.
- **Sound:** WAV loops until dismissed (Space / button).
- **Video:** MP4 in centre panel during alarm — requires **ffmpeg** on PATH.
- **Chimes:** place WAVs in `sounds/` — see `sounds/README.txt`.

Full field reference: [CUSTOMIZATION.md — alarms](CUSTOMIZATION.md#configalarmscsv--alarm-schedule).

---

## ESP8266 serial

Optional WiFi/HTTP coprocessor on UART.

| Topic | Doc |
|-------|-----|
| `esp8266.conf` keys | [CUSTOMIZATION.md](CUSTOMIZATION.md#configesp8266conf--wifi-serial-bridge) |
| Wiring, protocol, flash, debug | [DRIVERS.md — ESP8266](DRIVERS.md#esp8266-wifi-bridge) |

### Permissions

```bash
sudo usermod -aG dialout $USER
# log out and back in
```

When `enabled=true`, weather HTTP tries the bridge first, then host `ureq`.
Console: `[esp8266] opened …`, `[esp8266] bridge online`, or
`[esp8266] not available: …` (graceful fallback).

---

## Build and run

```bash
./scripts/linux-build.sh
cargo run --features linux-full

# Release
./scripts/linux-build.sh --release
```

Binary: `target/debug/pi-smart-clock` (or `release/`).

**Do not** enable `pico-dvi` alongside `linux-full` — features are mutually exclusive ([SHARED_CODE.md](SHARED_CODE.md#cargo-features)).

---

## Rust toolchain note

Linux builds work with **rustup** or distro `cargo`/`rustc`.  
Pico cross-builds need rustup only — see [EMBEDDED.md](EMBEDDED.md#development-host-toolchain).

If both apt and rustup are installed, prefer `~/.cargo/bin` for consistency:

```bash
source ~/.cargo/env
which cargo   # should not be /usr/bin/cargo for embedded work
```

---

## Troubleshooting

| Issue | Remedy |
|-------|--------|
| SDL / mixer link errors | Install `libsdl2-dev`, `libsdl2-mixer-dev`, `libsdl2-ttf-dev` |
| Alarm video silent / blank | Install `ffmpeg`; check `video_file` path |
| No chime sounds | Add WAVs under `sounds/`; check SDL_mixer init in stderr |
| ESP8266 permission denied | `dialout` group, correct `port=` |
| Config ignored | Check `~/.config/pi-smart-clock/` overrides repo `config/` |
| Parse errors after edit | `rm -rf ~/.config/pi-smart-clock` |

Config-specific messages: [CUSTOMIZATION.md — errors](CUSTOMIZATION.md#config-errors-and-remedies).