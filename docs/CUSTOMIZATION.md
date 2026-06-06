# Customization Manual

Everything you can change **without recompiling** (config files, media, assets) plus how to extend faces and modules in code.

```
    FILE TYPES COVERED
    ------------------
    *.conf / *.csv     User settings (weather, panels, alarms, WiFi, face name)
    assets/            SVG clock faces, fonts, weather icons
    sounds/ videos/    Chimes and alarm media
```

Platform-specific **where files live**:

| Content | Linux | Embedded (SD card) |
|---------|-------|---------------------|
| `*.conf` | `~/.config/pi-smart-clock/` → `config/` → `*.example` | `/sd/config/` |
| `alarms.csv` | `~/.local/share/pi-smart-clock/config/` | `/sd/config/` |
| Cache JSON | `~/.cache/pi-smart-clock/` | `/sd/cache/` |
| Sounds / video | Repo `sounds/`, `videos/` (or paths in CSV) | `/sd/sounds/`, `/sd/videos/` |

Linux search order and recovery: [LINUX.md](LINUX.md#persistence).  
Embedded SD layout: [EMBEDDED.md](EMBEDDED.md#sd-card-layout).

---

## Configuration file format

All `*.conf` files use line-oriented `key=value` syntax.

```
    key=value          Assign a parameter
    # comment          Ignored
    blank line         Ignored

    Keys: case-insensitive. Whitespace around keys/values is trimmed.
```

**Booleans** (`enabled=` and similar): `1`, `true`, `yes`, `on` → true; anything else → false.

Copy examples before editing:

```bash
cp config/esp8266.conf.example config/esp8266.conf
cp config/weather.conf.example  config/weather.conf
cp config/panels.conf.example   config/panels.conf
cp config/faces.conf.example    config/faces.conf
cp config/alarms.csv.example    config/alarms.csv
```

---

## `config/esp8266.conf` — WiFi serial bridge

Routes WiFi, HTTP, NTP, and MQTT through an ESP8266 on UART (3.3 V).

| Parameter | Default | Description |
|-----------|---------|-------------|
| `enabled` | `false` | Use ESP8266 for network I/O when true |
| `port` | `auto` | Serial device (`/dev/ttyUSB0`, `auto`, etc.) |
| `baud` | `115200` | Must match bridge firmware |
| `wifi_ssid` | *(empty)* | Synonyms: `ssid` |
| `wifi_password` | *(empty)* | Synonyms: `wifi_pass`, `password` |

**Full driver guide:** wiring, protocol, flashing, troubleshooting —
[DRIVERS.md — ESP8266](DRIVERS.md#esp8266-wifi-bridge).

**Linux:** add user to `dialout` — [LINUX.md](LINUX.md#esp8266-serial).

---

## `config/faces.conf` — Clock face selection

| Parameter | Default | Description |
|-----------|---------|-------------|
| `face` | `retro-roman` | Face module id |

| Value | Asset path |
|-------|------------|
| `retro-roman`, `retro_roman`, `default` | `assets/faces/retro-roman/face.svg` |

Unknown values fall back to `retro-roman`.

### Clock faces (SVG assets)

```
    assets/faces/<face-name>/face.svg           Dial + numeral glyphs
    assets/faces/<face-name>/hour-hand.svg      Hour hand (points to 12 o'clock)
    assets/faces/<face-name>/minute-hand.svg    Minute hand
    assets/faces/<face-name>/second-hand.svg    Second hand
    assets/faces/<face-name>/hub.svg            Centre cap (optional)
    assets/fonts/DejaVuSerif-Bold.ttf           Roman numeral glyphs (shared)
```

**Hand SVG rules**

- Use a **512×512** `viewBox` with pivot at the centre **(256, 256)**.
- Wrap artwork in `<g id="hand">…</g>`.
- Draw the hand pointing **up** (toward 12 o'clock) — rotation is applied at runtime.
- Set `design_length` in `FaceLayout` (`src/modules/faces/layout.rs`) to the SVG
  distance from pivot to tip (see `retro-roman` for reference).

**Adding a new face** requires:

1. Create the SVG set under `assets/faces/my-face/`.
2. Register `FaceId` in `src/modules/faces/mod.rs` (`parse()`, `asset_path()`, `layout()` with hand files and `design_length` values).
3. Set `face=my-face` in `faces.conf`.

Linux renders faces with **resvg** at runtime. Embedded face SVG support follows the Linux module set as it lands on Pico.

---

## `config/panels.conf` — Bottom panel modules

Assigns modules to the three bottom slots (portrait layout):

```
    ┌─────────────────────────────────────┐
    │           CLOCK FACE                │
    ├───────────┬───────────┬─────────────┤
    │  b_left   │   b_mid   │   b_right   │
    └───────────┴───────────┴─────────────┘
```

| Slot key | Position |
|----------|----------|
| `b_left` | Left panel |
| `b_mid` | Centre |
| `b_right` | Right |

| Module value | Panel content |
|--------------|---------------|
| `weather` | Open-Meteo temperature, conditions, AQI |
| `calendar` | Upcoming events |
| `holidays` | Upcoming holidays |

All three slots are **required**. Each module may appear once. Default:

```
b_left=weather
b_mid=calendar
b_right=holidays
```

**Adding a module** requires a new `ModuleId` variant and panel implementation in `src/modules/` (see [SHARED_CODE.md](SHARED_CODE.md#bottom-panel-modules)).

---

## `config/weather.conf` — Weather source

Open-Meteo — no API key. [https://open-meteo.com/](https://open-meteo.com/)

| Parameter | Default | Synonyms |
|-----------|---------|----------|
| `latitude` | `39.7684` | `lat` |
| `longitude` | `-86.1581` | `lon`, `lng` |
| `timezone` | `auto` | `tz` |
| `units` | `fahrenheit` | `temperature_unit` |
| `update_interval_minutes` | `30` | `update_minutes`, `interval` |
| `city` | *(lookup)* | `location` |

**Units:** `fahrenheit`/`f`/`farenheit` → °F; `celsius`/`c`/`centigrade` → °C.

**City name:** if `city` is unset, reverse geocode via Nominatim; cached in `cache/weather_city.json` (1 hour).

**Hot reload (Linux):** changes to `weather.conf` are picked up without restart.

### Weather icons (SVG)

```
    assets/icons/yaru/<icon-name>.svg
```

Icons are chosen automatically from WMO weather codes. Add Yaru-style symbolic SVGs under that tree to extend the set; filenames must match the resolver in `src/icons/`.

---

## `config/alarms.csv` — Alarm schedule

CSV, one alarm per row. Max **4 alarms** (id 0–3).

**Header (required):**

```
id,hour,minute,enabled,repeat,label,sound_file,video_file,snooze_minutes
```

| Field | Range | Description |
|-------|-------|-------------|
| `id` | 0–3 | Slot index |
| `hour` | 0–23 | 24-hour local time |
| `minute` | 0–59 | |
| `enabled` | true/false | Armed when true |
| `repeat` | true/false | Daily repeat |
| `label` | string | Display name |
| `sound_file` | path | WAV, loops until dismissed |
| `video_file` | path | MP4 (Linux: centre panel via ffmpeg) |
| `snooze_minutes` | int | Reserved (default 9) |

**Legacy 8-column format** (no `video_file`) is still accepted.

**Media resolution order:**

1. Exact path if it exists  
2. `sounds/<filename>` or `videos/<filename>`  
3. Repo / manifest directory  

**Persistence when saved from menu:**

| Platform | Path |
|----------|------|
| Linux | `~/.local/share/pi-smart-clock/config/alarms.csv` |
| Linux backups | `~/.local/state/pi-smart-clock/backups/alarms_*.csv.bak` |
| Embedded | `/sd/config/alarms.csv` and `/sd/config/alarms_*.csv.bak` |

---

## Sounds and video (not in `*.conf`)

### Chimes (automatic)

| Event | Default path |
|-------|--------------|
| Tick | `sounds/tick.wav` |
| Tock | `sounds/tock.wav` |
| Quarter hour | `sounds/quarter.wav` |
| Half hour | `sounds/half.wav` |
| Hour | `sounds/bell.wav` |

See `sounds/README.txt`. Chimes are **Linux-only** today (`linux-full` + SDL_mixer).

### Alarm media

- **Sounds:** `sounds/*.wav` — referenced from `alarms.csv`  
- **Video:** `videos/*.mp4` — Linux playback via **ffmpeg**; see `videos/README.txt`

On embedded, copy media to `/sd/sounds/` and `/sd/videos/` and use matching paths in `/sd/config/alarms.csv`.

---

## Fonts

| File | Use |
|------|-----|
| `assets/fonts/DejaVuSerif-Bold.ttf` | Roman numerals on SVG clock face (Linux) |

System UI on Linux also uses SDL2_ttf with DejaVu (install `fonts-dejavu-core` on Debian Trixie / Pi OS — included in `./scripts/linux-deps.sh`).

---

## Runtime cache (do not edit)

Written by the program; delete to force refresh.

| Virtual path | Linux | Purpose |
|--------------|-------|---------|
| `cache/weather_city.json` | `~/.cache/pi-smart-clock/` | Geocode city name |
| `cache/weather_data.json` | `~/.cache/pi-smart-clock/` | Forecast snapshot |

Embedded: `/sd/cache/`. Invalidation tied to coordinates, config mtime, and `update_interval_minutes`.

---

## Quick reference — all customizable files

```
┌─────────────────────┬──────────────────────────────────┬─────────────────────┐
│ FILE                │ KEY (SYNONYMS)                   │ DEFAULT             │
├─────────────────────┼──────────────────────────────────┼─────────────────────┤
│ esp8266.conf        │ enabled, port, baud              │ false, auto, 115200 │
│                     │ wifi_ssid (ssid)                 │ ""                  │
│                     │ wifi_password (password, …)      │ ""                  │
├─────────────────────┼──────────────────────────────────┼─────────────────────┤
│ faces.conf          │ face                             │ retro-roman         │
├─────────────────────┼──────────────────────────────────┼─────────────────────┤
│ panels.conf         │ b_left, b_mid, b_right           │ weather, calendar,  │
│                     │                                  │ holidays            │
├─────────────────────┼──────────────────────────────────┼─────────────────────┤
│ weather.conf        │ latitude (lat), longitude (lon)  │ 39.7684, -86.1581   │
│                     │ timezone (tz), units             │ auto, fahrenheit    │
│                     │ update_interval_minutes          │ 30                  │
│                     │ city (location)                  │ *(geocode)*         │
├─────────────────────┼──────────────────────────────────┼─────────────────────┤
│ alarms.csv          │ id, hour, minute, enabled, …     │ see table above     │
└─────────────────────┴──────────────────────────────────┴─────────────────────┘
```

---

## Config errors and remedies

| Message | Fix |
|---------|-----|
| `[panels] missing slot assignment(s)` | Set `b_left`, `b_mid`, `b_right` in `panels.conf` |
| `[panels] unknown module` | Use `weather`, `calendar`, or `holidays` only |
| `[weather] bad latitude/longitude/units` | Fix numeric/format in `weather.conf` |
| `[faces] no config found, using default` | Copy `faces.conf.example` or set `face=` |
| Garbled `~/.config/pi-smart-clock/` | `rm -rf ~/.config/pi-smart-clock` and restart |
| `[esp8266] not available` | [DRIVERS.md — troubleshooting](DRIVERS.md#troubleshooting) |

Embedded build errors: [EMBEDDED.md](EMBEDDED.md#troubleshooting).