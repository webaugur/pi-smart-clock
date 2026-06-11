# Desktop / Unix Guide

SDL2 desktop build for development, kiosk images, **Debian 13 Trixie**, **Raspberry Pi OS Trixie**, and **OpenIndiana 2025**.

The project no longer has an embedded/Pico target. The single "full" desktop build (SDL2 + supporting crates) is the only supported configuration and targets modern Unix desktops on the two reference OSes.

We treat **Debian Trixie** and **OpenIndiana 2025** as the primary tested platforms. Other Linux or illumos systems may work with appropriate dependencies.

**Supported architectures**
- Debian 13 (Trixie) / Raspberry Pi OS Trixie: **amd64 (x64) and arm64 (ARM64/aarch64)**. Both are first-class; the same `libsdl2-dev` etc. packages work on either.
- OpenIndiana 2025: **amd64 (x64) primary**. arm64 support in OI/illumos is limited/experimental as of 2025 — build natively on an arm64 OI system (when pkgsrc is available for it) or cross-compile from amd64. The desktop code itself is portable.

For config files and assets see [CUSTOMIZATION.md](CUSTOMIZATION.md).  
For shared Rust layout see [SHARED_CODE.md](SHARED_CODE.md).

---

## Quick start

**Debian Trixie / Pi OS Trixie (apt):**

```bash
git clone https://github.com/webaugur/pi-smart-clock.git
cd pi-smart-clock
git checkout full-project

./scripts/linux-deps.sh
./scripts/linux-build.sh
cargo run --features full
# or simply: cargo run  (full is the default)
```

**OpenIndiana 2025 (pkgsrc / pkgin example):**

```bash
# Ensure pkgsrc is bootstrapped and in PATH
pkgin install sdl2 sdl2-ttf sdl2-mixer ffmpeg git rust
# (adjust package names for your OI 2025 pkgsrc collection; noto fonts for CJK)
git clone https://github.com/webaugur/pi-smart-clock.git
cd pi-smart-clock
git checkout full-project

# No special -deps script yet — the above + system equivalents for build-essential/pkg-config
cargo build --features full
cargo run
```

See also font notes below for CJK on OI.

### Docker / CI

Reproducible Trixie build (no local SDL packages needed). The image supports both amd64 and arm64:

```bash
# Multi-arch (recommended)
docker buildx build --platform linux/amd64,linux/arm64 -t pi-smart-clock --push .

# Single-arch (current host)
docker build -t pi-smart-clock .
```

See the updated `Dockerfile` for buildx examples.

GitHub Actions (amd64 by default) uses `debian:trixie-slim` — see `.github/workflows/linux-trixie.yml`. Arm64 package builds are best done natively on an arm64 Debian host or via QEMU/binfmt in CI. The workflow now uses dynamic architecture for .deb names.

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

**(The project is now a single desktop target; there is no separate `pico-dvi` feature.)

---

## Rust toolchain note

Desktop builds work with **rustup** or distro `cargo`/`rustc` on the supported Unix platforms. No cross-compilation for embedded targets is required (Pico support has been removed).

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