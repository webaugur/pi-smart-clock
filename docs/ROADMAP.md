# Roadmap

High-level milestones for Pi Smart Clock. Granular tasks, file paths, and status live in **[TODO.md](TODO.md)** — update that doc when closing items.

Run `./scripts/audit-todos.sh` to check inline `// TODO(ID):` comments against the tracker.

---

## Current state (2026-06-06)

| Platform | Status |
|----------|--------|
| **Linux (`linux-full`)** | Dev-ready: SDL clock face, SVG hands, weather panel, alarms, menu, ESP8266 bridge |
| **Pico 1 (`pico-dvi`)** | DVI clock face on 640×480; SD/RTC/network still stubs |
| **Pico 2 (RP2350)** | Not started — see [PLAT-001](TODO.md#p3--future--platform-expansion) |

---

## Milestone 1 — Pico boots as a real clock

**Goal:** DVI output, correct time, SD config loads.

| Checkpoint | TODO IDs | Status |
|------------|----------|--------|
| DVI framebuffer + clock face drawn | PICO-001 | done (2026-06-06) |
| SD FAT mount + read/write | PICO-002 | open |
| Alarms CSV on `/sd/config/` | PICO-003 | open |
| DS3231 wall time | HW-003, PICO-004 | open |
| Honest boot screen | PICO-005 | open |

**Unblocks:** factory config, chimes, alarm sounds from SD.

---

## Milestone 2 — Connected device

**Goal:** WiFi bridge, input, sensors, persisted time-set.

| Checkpoint | TODO IDs | Status |
|------------|----------|--------|
| ESP8266 UART on Pico | NET-001 | open |
| MQTT / NTP via bridge | NET-002, NET-003 | open |
| Rotary encoder + button GPIO | HW-001 | open |
| AHT20 temp/humidity | HW-002 | open |
| Time-set writes RTC | HW-004 | open |
| SD WAV alarm playback | AUDIO-001 | open |

**Unblocks:** weather fetch on device, menu navigation without SDL keys.

---

## Milestone 3 — Voice, OTA, alerts, expansion

**Goal:** README “in progress” features and platform growth.

| Checkpoint | TODO IDs | Status |
|------------|----------|--------|
| Voice feedback + commands | VOICE-001, VOICE-002, VOICE-003 | open |
| OTA with rollback | OTA-001 | open |
| NWS alerts + radar | UI-003, UI-004, UI-005 | open |
| Live calendar / holidays (Linux) | UI-001, UI-002 | open |
| Web status / alarm UI | WEB-001 | open |
| Pico 2 firmware | PLAT-001 | open |
| SD SPI/SDIO, embedded SVG faces, Pico W WiFi | PLAT-002, PLAT-003, PLAT-004 | open |

---

## Platform matrix (summary)

| Area | Linux | Pico 1 | Pico 2 |
|------|-------|--------|--------|
| Clock face + SVG hands | Yes | DVI (basic analog; SVG → PLAT-003) | Planned |
| Bottom panels | Yes (weather live; cal/hol samples) | N/A | — |
| ESP8266 WiFi | Yes | Not built | TBD |
| SD / config | XDG paths | Stub I2C SD | Same layout |
| RTC (DS3231) | Stub | Stub | Planned |
| Encoder / button | SDL keys | GPIO stub | Planned |
| MQTT / OTA / voice | Partial / stub | Stub | Planned |

Driver detail: [DRIVERS.md](DRIVERS.md). Item-level tracking: [TODO.md](TODO.md).

---

## Maintenance

Same change as the code — see [AGENTS.md](../AGENTS.md).

When all IDs for a milestone checkpoint are `done` in [TODO.md](TODO.md), set that checkpoint to **done** here with a date.

When adding large new work, add a row to [TODO.md](TODO.md) first, tag `// TODO(ID):` in code, then reference the ID from this file. Run `./scripts/audit-todos.sh` before closing the task.