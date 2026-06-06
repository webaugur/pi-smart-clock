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