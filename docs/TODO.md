# Remaining work — detailed tracker

Actionable checklist for open issues, stubs, and planned features. For phased milestones see **[ROADMAP.md](ROADMAP.md)**.

Run `./scripts/audit-todos.sh` before releases to compare inline `// TODO(ID):` comments against this file.

---

## How to maintain

**Agents and contributors:** follow [AGENTS.md](../AGENTS.md) — tracker updates are **required in the same PR/change** as the code fix, not afterwards.

### Closing an item (checklist)

| Step | Action |
|------|--------|
| 1 | Implement the fix (or mark `in-progress` if partial). |
| 2 | Remove all `// TODO(ID):` comments for that ID in `src/` / `firmware/`. |
| 3 | In this file: set **Status** → `done`, add date, move row to [Completed](#completed), refresh [Summary](#summary) counts. |
| 4 | In [ROADMAP.md](ROADMAP.md): mark milestone checkpoints `done` when all their IDs are closed. |
| 5 | Run `./scripts/audit-todos.sh` — resolve any **warn** lines before finishing. |
| 6 | Update [DRIVERS.md](DRIVERS.md) status one-liners if driver behavior changed. |

### Opening new work

1. Add a row here with a new ID before or with the stub code.
2. Tag code with `// TODO(ID): description`.
3. Link the ID from [ROADMAP.md](ROADMAP.md) if it affects a milestone.

**Status legend:** `open` · `in-progress` · `done` · `wontfix`

**Platform tags:** `linux-full` · `pico-dvi` · `both`

**Code convention:** use `// TODO(ID): description` in Rust when the item has an ID here (enables the audit script).

---

## Summary

| Priority | Open | Focus |
|----------|------|-------|
| P0 | 4 | Pico SD, wall time (DVI done) |
| P1 | 11 | Network, voice, OTA, GPIO, sensors, RTC |
| P2 | 6 | Linux panels, alerts, web UI |
| P3 | 4 | Pico 2, buses, embedded faces, Pico W WiFi |

*Last full audit: 2026-06-06*

---

## P0 — Pico cannot run as a real clock yet

| ID | Status | Platform | Title | Key files |
|----|--------|----------|-------|-----------|

| PICO-002 | open | pico-dvi | SD card mount + FAT read/write | `src/drivers/sd_storage.rs` |
| PICO-003 | open | pico-dvi | Load/save alarms from `/sd/config/` | `src/clock_core/persistence.rs` (blocked by PICO-002) |
| PICO-004 | open | pico-dvi | Real wall time (not boot counter) | `src/platform/rp2040.rs`, `src/drivers/ds3231.rs` |
| PICO-005 | open | pico-dvi | Boot screen claims RTC sync while DS3231 is empty | `src/clock_core/boot_screen.rs` (blocked by HW-003) |

### PICO-002 — SD storage

`SdStorage::mount` sets `mounted = true` without probing card. `read_file` / `write_file` return "not yet implemented". Default bus: I2C (`StorageBusMode::I2c`).

**Blocks:** PICO-003, config load, sounds, videos, cache on device.

### PICO-003 — Alarm persistence on SD

`persistence::load_alarms` / `save_alarms` call `platform.read_file` / `write_file` → SD stub on Pico.

### PICO-004 — Wall time

`WALL_SECONDS` in `rp2040.rs` increments from a fixed 07:00:00 seed. No DS3231 or NTP integration on embedded.

### PICO-005 — Misleading boot text

`boot_screen::show` prints "RTC Synced" after `DS3231::synchronize` which is an empty stub.

---

## P1 — Core device features (README in progress)

| ID | Status | Platform | Title | Key files |
|----|--------|----------|-------|-----------|
| NET-001 | open | pico-dvi | Build ESP8266 driver on Pico | `src/drivers/mod.rs` |
| NET-002 | open | pico-dvi | MQTT / NTP Platform hooks on Pico | `src/drivers/platform.rs`, `src/drivers/mqtt.rs`, `src/drivers/ntp.rs` |
| NET-003 | open | both | Wire NtpClient / MqttClient into runtime | `src/runtime/tick.rs`, `src/runtime/state.rs` |
| VOICE-001 | open | both | Map voice messages to SD WAV files | `src/clock_core/voice_feedback.rs` |
| VOICE-002 | open | both | Wire `voice_commands::process` into tick loop | `src/clock_core/voice_commands.rs` |
| VOICE-003 | open | pico-dvi | I2S microphone / energy threshold | `src/drivers/microphone.rs` |
| OTA-001 | open | both | OTA download, flash, rollback | `src/ota/updater.rs`, `src/ota/rollback.rs` |
| HW-001 | open | pico-dvi | GPIO rotary encoder + push button | `src/drivers/rotary_encoder.rs`, `src/platform/rp2040.rs` |
| HW-002 | open | both | AHT20 + env sensor read path | `src/drivers/aht20.rs`, `src/clock_core/sensors.rs` |
| HW-003 | open | both | DS3231 I2C read/write | `src/drivers/ds3231.rs` |
| HW-004 | open | both | Time-set UI persists to RTC | `src/clock_core/time_set_ui.rs` |

### NET-001 — ESP8266 on Pico

`esp8266` module is `#[cfg(feature = "linux-full")]` only. Pico firmware cannot use the serial bridge today.

### NET-002 / NET-003 — Network stack

On Pico, `esp8266_mqtt_*`, `esp8266_get_ntp`, `http_download_binary` are trait defaults (no-op / `None`). `NtpClient::sync` and `MqttClient` exist but are never called from `tick` or `init`.

### VOICE-001 — Voice feedback

Crude string→path heuristic; no SD-backed WAV catalog. `Platform::speak` is a no-op on Pico.

### VOICE-002 / VOICE-003 — Voice input

`voice_commands::process` and `VoiceInput::listen` are implemented but not invoked from the main loop.

### OTA-001 — Over-the-air updates

`OtaUpdater::check_and_update` is empty when enabled. Embedded `OtaUpdater` in `state.rs` is a stub struct. `flash_*` / `reboot` Platform hooks are no-ops.

### HW-001 — Encoder input

`RotaryEncoder::update` reads `read_rotary_delta` / `read_pushbutton` — always `0` / `false` on Pico defaults. Linux uses SDL keys.

### HW-002 — Environment sensor

`Aht20Sensor::read` returns hardcoded `(23.7, 51.2)`. `EnvSensor::read` uses placeholder values.

### HW-003 / HW-004 — RTC

`DS3231::synchronize` and `set_time` are empty. `TimeSetUI` edits local fields only; does not call `set_time` or persist.

---

## P2 — Linux polish and shared stubs

| ID | Status | Platform | Title | Key files |
|----|--------|----------|-------|-----------|
| UI-001 | open | linux-full | Live calendar data (not samples) | `src/modules/calendar.rs` |
| UI-003 | open | both | NWS alert fetch | `src/clock_core/alerts.rs` |
| UI-004 | open | both | Weather radar imagery | `src/clock_core/panels/weather.rs`, `src/clock_core/weather.rs` |
| UI-005 | open | both | Alert photo download + BMP save | `src/clock_core/alert_photos.rs` |
| WEB-001 | open | linux-full | Wire web server; finish alarms page | `src/web/esp_web.rs` |
| AUDIO-001 | open | both | SD WAV playback on embedded | `src/drivers/sd_audio.rs` |

### UI-001 / UI-002 — Bottom panels

`HolidaysPanel` now computes live upcoming public holidays (multiple countries via `config/holidays.conf`, date-aware using chrono on Linux). `CalendarPanel` still ships hardcoded sample events. The `Platform::fetch_*` hooks remain no-ops (panels are self-updating where implemented).

### UI-003 — NWS alerts

`AlertManager::check_nws_alerts` only rate-limits; no API fetch (except manual `force_radar`).

### UI-004 — Radar overlay

Placeholder rectangle + text; no live radar tiles.

### UI-005 — Alert photos

`fetch_photo` falls back to `create_official_placeholder` (Platform no-op on Pico).

### WEB-001 — Web UI

`WebServer` module exists but is not started from `main` or `tick`. Alarms page shows "coming soon".

### AUDIO-001 — SD audio

`play_wav_from_sd` forwards to `play_raw_audio` (empty on Pico).

---

## P3 — Future / platform expansion

| ID | Status | Platform | Title | Key files |
|----|--------|----------|-------|-----------|
| PLAT-001 | open | pico-dvi | Pico 2 (RP2350) firmware profile | `Cargo.toml`, `docs/EMBEDDED.md` |
| PLAT-002 | open | pico-dvi | SD SPI / SDIO bus modes | `src/drivers/sd_storage.rs` |
| PLAT-003 | open | pico-dvi | SVG face rendering on embedded | `src/modules/faces/` |
| PLAT-004 | open | pico-dvi | Pico W on-chip WiFi (replace ESP8266) | `docs/EMBEDDED.md` |

---

## Orphan integrations

Modules implemented but **not called** from `src/runtime/tick.rs` or `src/runtime/state.rs::init`:

| Module | File | Related ID |
|--------|------|------------|
| `NtpClient` | `src/drivers/ntp.rs` | NET-003 |
| `MqttClient` | `src/drivers/mqtt.rs` | NET-003 |
| `VoiceInput` | `src/drivers/microphone.rs` | VOICE-003 |
| `voice_commands::process` | `src/clock_core/voice_commands.rs` | VOICE-002 |
| `WebServer` | `src/web/esp_web.rs` | WEB-001 |

---

## Inline code TODOs

| ID | File | Line (approx.) |
|----|------|----------------|
| PICO-002 | `src/drivers/sd_storage.rs` | mount, read, write |
| HW-002 | `src/clock_core/sensors.rs` | AHT20 / DS3231 read |
| VOICE-001 | `src/clock_core/voice_feedback.rs` | WAV mapping |

---

## Completed

| ID | Completed | Notes |
|----|-----------|-------|
| PICO-001 | 2026-06-06 | DVI via `pico-dvi-rs` (640×480 VGA), `dvi_gfx` display lists, Pico DVI Sock pinout |
| UI-002 | 2026-06-10 | Live global holidays panel (computed, multi-country via config/holidays.conf; replaced all samples). Calendar (UI-001) still pending. |

---

## Related docs

| Doc | Role |
|-----|------|
| [ROADMAP.md](ROADMAP.md) | Milestone phases |
| [DRIVERS.md](DRIVERS.md) | Wiring and protocol reference |
| [EMBEDDED.md](EMBEDDED.md) | Pico build, flash, hardware |
| [SHARED_CODE.md](SHARED_CODE.md) | Architecture and feature flags |