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

**Last significant work:** Pico target removal + retarget to OpenIndiana 2025 and Debian Trixie (desktop-only).
- Removed all Pi Pico / RP2040 / DVI / embedded support: `pico-dvi` feature, `firmware/`, `third_party/pico-dvi-rs/`, `vendor-pico-dvi-src/`, `memory.x`, pico scripts, `src/platform/{dvi_gfx.rs,rp2040.rs}`, all `pico-dvi` / old `linux-full` cfgs, no_std paths, etc.
- Simplified to a single desktop "full" feature / build (SDL2 on Unix).
- Updated Cargo, build system, scripts (generalized for OI + Trixie), and all docs.
- Tracker: all PICO-* and pico-specific items set to `wontfix`; inline TODOs removed; Summary and sections updated (same change + audit).
- AGENTS, ROADMAP, TODO, README, LINUX.md, etc. rewritten for the new desktop focus on **Debian Trixie** and **OpenIndiana 2025**.
- Playful icon set (from prior work) and other desktop features (holidays, upper modules, boot) preserved.

**Where we left off / active goal:**
- Desktop SDL2 clock on two reference Unix platforms: Debian 13 Trixie (apt + debian/ packaging) and OpenIndiana 2025 (pkgsrc notes + portable build).
- No embedded/Pico target remains. Linux (`full` / desktop) is the *only* target.
- Focus on portability (fonts, serial, persistence, deps), polish of P2 items, and clean docs/tracker reflecting the removal.
- See updated [docs/ROADMAP.md](docs/ROADMAP.md) for new milestones and [docs/TODO.md](docs/TODO.md) (P0/P3 cleared, P1/P2 desktop-focused).

**Fast re-orientation checklist (run these first):**
1. Read [docs/TODO.md](docs/TODO.md) — note the reduced P0/P3, wontfix Pico items, and current P2 desktop work.
2. Read the current [docs/ROADMAP.md](docs/ROADMAP.md) (new platform matrix and milestones; Pico columns/milestones removed).
3. `./scripts/audit-todos.sh` — confirms tracker vs. inline comments (Pico TODOs cleaned).
4. `cargo run --features full` (or just `cargo run`) — exercise the desktop clock (Esc quit, M menu, arrows = encoder). Works on Trixie; follow LINUX.md for OI 2025.
5. Review current docs: [docs/LINUX.md](docs/LINUX.md) (now covers both Trixie and OI), README, AGENTS (this section).

**Recent architecture notes:**
- Single desktop build. No more feature splits or no_std.
- Boot flow (in `clock_core/boot/`) is Linux/desktop-oriented (splash, reveal, loader).
- Platform is `src/platform/linux*` only (SDL).
- Storage uses XDG-style paths (works on both Trixie and OI).
- Many higher-level modules (NtpClient, MqttClient, VoiceInput, WebServer, etc.) exist but wiring is desktop-focused.
- Icon set is the playful cartoony one under `assets/icons/playful/` with hi/lo SVG support.

**Rules reminder (from top of this file):** Any change that touches tracked IDs must also update the trackers + run the audit **in the same change**. Do not leave that for "later".

**Canonical state lives in:** `docs/TODO.md` and `docs/ROADMAP.md`. This section is only for fast agent restart context and should be lightly maintained. (Pico context from prior sessions has been replaced.)