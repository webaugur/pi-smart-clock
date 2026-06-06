# Shared Code Architecture

Rust library used by **both** Linux (`linux-full`) and Pico (`pico-dvi`) binaries. Platform-specific code is gated by Cargo features.

| Doc | Topic |
|-----|-------|
| [CUSTOMIZATION.md](CUSTOMIZATION.md) | Config files and assets |
| [LINUX.md](LINUX.md) | SDL2 binary and XDG |
| [EMBEDDED.md](EMBEDDED.md) | Firmware binary and SD card |

---

## Repository layout

```
pi-smart-clock/
├── src/
│   ├── lib.rs              Crate root, feature gates
│   ├── clock_core/         UI logic, alarms, menu, weather panels (both targets)
│   ├── runtime/            Main loop, SmartClockState, tick()
│   ├── drivers/            Sensors, SD, ESP8266, Platform trait
│   ├── platform/           linux.rs (SDL) | rp2040.rs (Pico DVI)
│   ├── storage/            XDG (Linux) | /sd/ (embedded) path mapping
│   ├── layout.rs           Screen geometry (portrait / landscape)
│   ├── config.rs           Compile-time constants (MQTT, overclock)
│   ├── modules/            [linux-full] SVG faces, bottom panels
│   ├── clock/              [linux-full] SDL layout regions
│   ├── chimes/             [linux-full] Quarter-hour sounds
│   ├── icons/              [linux-full] Weather SVG icons
│   ├── ota/                [linux-full] OTA stubs
│   └── web/                [linux-full] Status page stubs
├── firmware/
│   ├── main.rs             Pico entry (embassy executor)
│   └── alloc.rs            Bump allocator for RP2040
├── src/main.rs             Linux entry (SDL event loop)
└── config/                 Shipped defaults (*.example)
```

---

## Cargo features

| Feature | Default | Purpose |
|---------|---------|---------|
| `linux-full` | yes | SDL2, chrono, serde, ureq, resvg, serialport, desktop binary |
| `pico-dvi` | no | Embassy, RP2040, embedded-graphics, firmware binary |

**Mutually exclusive.** `build.rs` and `lib.rs` emit an error if both are enabled.

```bash
# Linux
cargo run --features linux-full

# Pico
cargo build --no-default-features --features pico-dvi --target thumbv6m-none-eabi
```

### Dependency split

| Area | linux-full | pico-dvi |
|------|------------|----------|
| Time | `chrono` | `timing`, `time_util::WallTime` |
| Strings / vec | `std` | `alloc` + `prelude` |
| Graphics | SDL2 + resvg | `Platform` draw calls (stub on Pico) |
| JSON config | `serde_json` | Not loaded on embedded yet (SD TODO) |
| HTTP | `ureq` (+ ESP8266) | ESP8266 / stubs |

---

## `clock_core` module

Renamed from `core` to avoid shadowing Rust's `::core` crate (fixes embedded dependency build errors with `nb` / `byteorder`).

Shared application logic:

| Module | Role |
|--------|------|
| `alarm`, `alarm_ui` | Schedule, ringing overlay |
| `alerts`, `panels/weather` | NWS-style alerts, radar panel |
| `menu`, `time_set_ui`, `about` | UI screens |
| `persistence` | Load/save alarms via `Platform` |
| `boot_screen`, `clock` | Splash and second-hand face update |
| `sensors`, `energy_monitor` | Environment telemetry |
| `update_scheduler` | Periodic refresh cadence |
| `weather` | Weather panel state |

Linux-only submodules: `alarm_video`, `voice_feedback`, `voice_commands` (cfg-gated in `mod.rs`).

Entry flow: `runtime::SmartClockState` holds managers; `runtime::tick::tick()` drives each frame.

---

## `Platform` trait

Defined in `src/drivers/platform.rs`. Abstracts hardware for `clock_core` and `runtime`.

Implementations:

| Type | File | Feature |
|------|------|---------|
| `LinuxPlatform` | `platform/linux.rs` | `linux-full` |
| `PicoDviPlatform` | `platform/rp2040.rs` | `pico-dvi` |

Capabilities include display primitives, audio, wall time, delays, filesystem (`read_file` / `write_file`), GPIO (button, encoder), network fetch hooks, and OTA flash hooks (stubs).

Async methods use `async fn in trait` (allowed at crate level).

---

## Storage abstraction

Virtual paths used in shared code:

| Path | Meaning |
|------|---------|
| `/sd/config/...` | Config and alarms |
| `cache/...` | Weather JSON |
| `/sd/alerts/...` | Alert photos |

**Linux:** `storage/linux.rs` maps to XDG dirs under `~/.config`, `~/.local`, `~/.cache`.

**Embedded:** `storage/embedded.rs` prefixes `cache/` → `/sd/cache/`; SD I/O via `drivers/sd_storage.rs` (I2C FAT — in progress).

---

## `layout`

`layout.rs` — logical resolution from `Layout::init(w, h)`:

- Portrait reference: 960×1280 (Linux dev); Pico firmware uses 800×480 in `firmware/main.rs`.
- Computes clock centre, hand length, bottom panel rectangles, status bar.

Linux SDL scales window to monitor; embedded draws in logical coordinates.

---

## Bottom panel modules

**Linux only** (`src/modules/`):

```
modules/
├── config.rs       panels.conf loader
├── module_id.rs    weather | calendar | holidays
├── bottom_panels.rs  Draw three slots
├── weather/        Open-Meteo panel
├── calendar/       Events placeholder
└── holidays/       Holidays placeholder
```

Slot assignment: `config/panels.conf` — [CUSTOMIZATION.md](CUSTOMIZATION.md#configpanelsconf--bottom-panel-modules).

To add a module: extend `ModuleId`, implement panel in `src/modules/`, wire into `BottomPanelBar`.

---

## Clock faces (Linux)

`src/modules/faces/` — SVG rasterization via resvg:

- `config/faces.conf` → `FaceId`
- Assets: `assets/faces/<name>/face.svg`
- Numerals: `assets/fonts/DejaVuSerif-Bold.ttf`

Adding a face requires code changes to `FaceId` — see [CUSTOMIZATION.md](CUSTOMIZATION.md#clock-faces-svg-assets).

Embedded currently uses `clock_core::clock::update()` with `Platform::draw_clock_face` stubs on Pico.

---

## Drivers

See **[DRIVERS.md](DRIVERS.md)** for wiring, protocols, config, and per-driver
status (ESP8266, DS3231, AHT20, SD, encoder, microphone, NTP, MQTT).

Summary: drivers live in `src/drivers/` and are reached via `Platform` methods.

---

## Binaries

| Binary | Path | Feature |
|--------|------|---------|
| `pi-smart-clock` | `src/main.rs` | `linux-full` |
| `pi-smart-clock-firmware` | `firmware/main.rs` | `pico-dvi` |

Firmware: `#![no_std]`, bump allocator in `firmware/alloc.rs`, Embassy executor, calls `SmartClockState::init` + `tick` loop.

---

## Build scripts and aliases

| Item | Role |
|------|------|
| `build.rs` | Git hash env vars; feature conflict check; embedded target check |
| `.cargo/config.toml` | `cargo pico` alias, probe-run for RP2040 |
| `scripts/setup-embedded.sh` | rustup target install |
| `scripts/pico-build.sh` | PATH-safe Pico build |

---

## Extending the codebase

1. **Shared behaviour** → `clock_core/` or `runtime/`, use `Platform` for I/O.
2. **Linux-only UI** → `modules/`, `panel/`, `clock/` behind `linux-full`.
3. **Hardware** → `drivers/` + platform impl methods.
4. **User-tunable** → config under `config/` + loader in `modules/` or `storage/` — document in [CUSTOMIZATION.md](CUSTOMIZATION.md).
5. **New board** → new `platform/*.rs`, Cargo feature, target triple in `Cargo.toml`.

Keep `linux-full` and `pico-dvi` disjoint to preserve `no_std` on embedded.