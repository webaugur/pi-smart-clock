# Roadmap

High-level milestones for Pi Smart Clock. Granular tasks, file paths, and status live in **[TODO.md](TODO.md)** — update that doc when closing items.

Run `./scripts/audit-todos.sh` to check inline `// TODO(ID):` comments against the tracker.

---

## Current state (2026-06-11)

| Platform | Status |
|----------|--------|
| **Desktop (Debian Trixie + OpenIndiana 2025)** | Primary target. SDL2 full desktop build (clock, panels, icons, boot, etc.). Pico/embedded target removed. |

---

## Milestone 1 — Desktop clock on Debian Trixie and OpenIndiana 2025

**Goal:** Reliable SDL2 desktop experience (clock face, panels, persistence, input, basic networking) on the two reference Unix platforms after Pico target removal.

| Checkpoint | TODO IDs | Status |
|------------|----------|--------|
| Single desktop build (feature "full") | (Pico removal) | done (2026-06-11) |
| Build + run on Debian Trixie (deps, docker, packaging) | — | in progress / maintained |
| Build + run on OpenIndiana 2025 (pkgsrc notes, font/serial portability) | OI-001 (new) | in progress |
| Polish desktop features (panels, icons, boot, alarms) | P2 items | ongoing |

**Unblocks:** Broader Unix desktop use (no more embedded divergence).

---

## Milestone 2 — Connected desktop

**Goal:** WiFi/bridge, sensors, persisted settings, voice on desktop Unix.

(Shared items from old Milestone 2, now desktop-focused.)

| Checkpoint | TODO IDs | Status |
|------------|----------|--------|
| Wire NtpClient / MqttClient | NET-003 | open |
| Voice feedback + commands (desktop) | VOICE-001, VOICE-002 | open |
| AHT20 / DS3231 desktop paths | HW-002, HW-003 | open |
| Time-set persists | HW-004 | open |
| OTA with rollback | OTA-001 | open |

---

## Milestone 3 — Polish, alerts, web, future

(Old M3 items that were desktop or shared, minus Pico expansion.)

| Checkpoint | TODO IDs | Status |
|------------|----------|--------|
| NWS alerts + radar | UI-003, UI-004, UI-005 | open |
| Live calendar (desktop) | UI-001 | open |
| Web status / alarm UI | WEB-001 | open |
| (Pico 2 / embedded items abandoned with target removal) | (PLAT-00x etc.) | wontfix |

---

## Platform matrix (summary)

| Area | Debian Trixie | OpenIndiana 2025 |
|------|---------------|------------------|
| Clock face + SVG hands | Yes (SDL + resvg) | Yes (same) |
| Bottom panels (weather, calendar, holidays, upper row) | Yes (live where implemented) | Yes (same desktop code) |
| Persistence (XDG-style) | Yes (~/.config etc.) | Yes (compatible paths) |
| ESP8266 bridge / serial | Yes | Yes (device names may differ; see docs) |
| RTC / sensors (DS3231, AHT20) | Stub / desktop paths | Same |
| Encoder / button | SDL keys | Same |
| MQTT / OTA / voice | Partial / in progress | Same desktop code |
| Packaging / deps | debian/ + apt scripts | pkgsrc/pkgin notes + build instructions |

Pico / embedded columns removed with the target.

Driver detail: [DRIVERS.md](DRIVERS.md). Item-level tracking: [TODO.md](TODO.md).

---

## Maintenance

Same change as the code — see [AGENTS.md](../AGENTS.md).

Pico-related milestones and items were marked abandoned/wontfix as part of the 2026-06-11 target removal (focus is now Debian Trixie + OpenIndiana 2025 desktop only).

When all IDs for a (current) milestone checkpoint are `done` in [TODO.md](TODO.md), set that checkpoint to **done** here with a date.

When adding large new work, add a row to [TODO.md](TODO.md) first, tag `// TODO(ID):` in code, then reference the ID from this file. Run `./scripts/audit-todos.sh` before closing the task.