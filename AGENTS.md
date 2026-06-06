# Agent instructions — Pi Smart Clock

When you fix, stub, or plan work that matches [docs/TODO.md](docs/TODO.md), **always update the tracker in the same change** (do not leave docs for a follow-up).

## Closing a tracked item (required)

If your change resolves all or part of an open ID (e.g. `PICO-002`, `HW-003`):

1. **Code** — Remove every `// TODO(ID):` comment for that ID in `src/` or `firmware/`. If only partially done, leave the comment and keep status `in-progress` in the doc.
2. **[docs/TODO.md](docs/TODO.md)** — Set status to `done`, add the completion date, move the row from the priority section to [Completed](docs/TODO.md#completed). Update the Summary open counts.
3. **[docs/ROADMAP.md](docs/ROADMAP.md)** — If every ID in a milestone checkpoint is `done`, mark that checkpoint complete and note the date.
4. **Audit** — Run `./scripts/audit-todos.sh` and fix any drift before finishing the task.
5. **Related docs** — If driver behavior changed, update the status line in [docs/DRIVERS.md](docs/DRIVERS.md) (do not duplicate full task lists there).

## Opening new work

If you add a stub, placeholder, or `// TODO` for non-trivial missing behavior:

1. Add a row to [docs/TODO.md](docs/TODO.md) with a new ID (`PICO-`, `NET-`, `HW-`, `UI-`, `VOICE-`, `OTA-`, `PLAT-`, `AUDIO-`, `WEB-`).
2. Use `// TODO(ID): description` in Rust when implementation is deferred.
3. Reference the ID from [docs/ROADMAP.md](docs/ROADMAP.md) if it affects a milestone.
4. Run `./scripts/audit-todos.sh`.

## Before you say a task is complete

- [ ] Tracker IDs touched by the change are `done` or still accurately `open` / `in-progress`
- [ ] `./scripts/audit-todos.sh` reports no errors (informational “open IDs without inline TODO” is OK)
- [ ] Pico and/or Linux build still passes if you changed `src/`

## Quick links

| Doc | Use |
|-----|-----|
| [docs/TODO.md](docs/TODO.md) | Item-level checklist |
| [docs/ROADMAP.md](docs/ROADMAP.md) | Milestones |
| [docs/DRIVERS.md](docs/DRIVERS.md) | Wiring and protocols |
| `./scripts/audit-todos.sh` | Drift check |

## Current context (for future session restarts)

**Branch:** `full-project`

**Last significant work (c8a9157):** New boot experience + Debian packaging for Trixie.
- Refactored boot into `src/clock_core/boot/` (loader.rs, mod.rs, reveal.rs, status.rs) + updated `boot_splash.rs`.
- Multi-phase boot: splash (video/image) → background loader steps → reveal transition (Linux) or direct to clock (Pico).
- Added full Debian packaging (`debian/`, scripts/debian-*.sh, desktop file, manpage).
- Updated Linux scripts and docs for Debian 13 (Trixie) target.

**Where we left off / active goal:**
- Linux (`linux-full`) is the primary dev target and is quite usable (SDL clock, SVG faces, weather, alarms, menu, new boot flow).
- Pico DVI graphics is working (PICO-001 complete).
- We are still in **Milestone 1**: "Pico boots as a real clock".
- The P0 blockers (see [docs/TODO.md](docs/TODO.md)) are the immediate focus:
  - **PICO-002** (SD card mount + FAT r/w over I2C) — biggest enabler; blocks config, alarms, sounds, etc.
  - **PICO-003** (alarm persistence on SD)
  - **PICO-004 + HW-003** (real wall time from DS3231 RTC instead of boot counter in `rp2040.rs`)
  - **PICO-005** (boot screen lies about "RTC Synced")

**Fast re-orientation checklist (run these first):**
1. Read [docs/TODO.md](docs/TODO.md) — pay special attention to the P0 section and the "Orphan integrations" table.
2. Read the current [docs/ROADMAP.md](docs/ROADMAP.md) milestone table.
3. `./scripts/audit-todos.sh` — confirms tracker vs. inline `// TODO(ID):` comments.
4. `cargo run --features linux-full` — exercise the current Linux experience (Esc quit, M menu, arrows = encoder).
5. For Pico context: look at `src/platform/rp2040.rs`, `src/drivers/sd_storage.rs`, `src/drivers/ds3231.rs`, and `src/clock_core/boot/mod.rs`.

**Recent architecture notes:**
- Boot flow was previously a single `boot_screen.rs`; now split for proper splash + progressive loading + nice reveal on Linux.
- `BootController` + `tick_boot()` live in `clock_core/boot/mod.rs` and drive loader steps while the splash runs.
- Platform trait has `show_boot_splash`, `finish_boot`, etc.
- Many higher-level modules (NtpClient, MqttClient, VoiceInput, WebServer, etc.) exist but are not yet wired into `runtime/tick.rs` or `state.rs::init` on all platforms.

**Rules reminder (from top of this file):** Any change that touches tracked IDs must also update the trackers + run the audit **in the same change**. Do not leave that for "later".

**Canonical state lives in:** `docs/TODO.md` and `docs/ROADMAP.md`. This section is only for fast agent restart context and should be lightly maintained.