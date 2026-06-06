
```
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║   ****  PI SMART CLOCK  ****                                                 ║
║                                                                              ║
║   CONFIGURATION REFERENCE MANUAL                                             ║
║   FOR LINUX AND EMBEDDED BUILDS                                              ║
║                                                                              ║
║   VERSION 1.0                          JUNE 2026                             ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
```


```
                            TABLE OF CONTENTS
                            -----------------

CHAPTER  1  -  BEFORE YOU BEGIN
CHAPTER  2  -  CONFIGURATION FILE FORMAT
CHAPTER  3  -  ESP8266 SERIAL BRIDGE  (config/esp8266.conf)
CHAPTER  4  -  CLOCK FACE SELECTION   (config/faces.conf)
CHAPTER  5  -  BOTTOM PANEL SLOTS     (config/panels.conf)
CHAPTER  6  -  WEATHER DATA SOURCE    (config/weather.conf)
CHAPTER  7  -  ALARM SCHEDULE         (config/alarms.csv)
CHAPTER  8  -  MEDIA AND ASSET PATHS
CHAPTER  9  -  RUNTIME CACHE FILES
CHAPTER 10  -  PI PICO 1 AND PI PICO 2 — INSTALLATION AND CONFIGURATION
APPENDIX A  -  QUICK REFERENCE TABLES
APPENDIX B  -  ESP8266 SERIAL COMMAND SET
APPENDIX C  -  ERROR MESSAGES AND REMEDIES
```


---

## CHAPTER 1 — BEFORE YOU BEGIN

The PI SMART CLOCK stores user settings in plain-text files under the
`config/` directory at the project root.  Copy each `*.example` file to
its live name before editing.

```
    OPERATION                          COMMAND
    ---------                          -------
    Copy ESP8266 settings              cp config/esp8266.conf.example config/esp8266.conf
    Copy weather settings              cp config/weather.conf.example config/weather.conf
    Copy panel assignments             cp config/panels.conf.example config/panels.conf
    Copy clock face selection          cp config/faces.conf.example config/faces.conf
    Copy alarm schedule                cp config/alarms.csv.example config/alarms.csv
```

**FILE SEARCH ORDER (LINUX)**

User-editable `*.conf` files are resolved in this order:

```
    1.  ~/.config/pi-smart-clock/<name>.conf
    2.  config/<name>.conf          (repo / working directory)
    3.  config/<name>.conf.example
```

If no file exists, built-in defaults are used and a message is printed to
the console (stderr).

**PERSISTENCE LAYOUT (LINUX)**

Runtime data uses XDG Base Directory paths under `~/.local` and `~/.cache`:

```
    ~/.config/pi-smart-clock/     User settings (weather, panels, faces, esp8266)
    ~/.local/share/pi-smart-clock/ Application data (alarms, alert photos)
    ~/.local/state/pi-smart-clock/ History (alarm backups)
    ~/.cache/pi-smart-clock/      Regenerable cache (weather JSON)
```

**DEPLOYMENT, BRANDING, AND RECOVERY**

The search order above is deliberate: you can ship a **default or branded**
configuration in the repo `config/` directory (or in your install package)
and it will run correctly on first launch — even before the end user has
touched anything.  When the user later saves settings, copies land in
`~/.config/pi-smart-clock/` and take precedence over the shipped defaults.

This split is useful for:

```
    • Factory / kiosk images with your city, panels, and face pre-set
    • OEM branding (default weather location, labels, module layout)
    • Support hand-off: repo defaults always remain as a known-good fallback
```

If a user's `~/.config/pi-smart-clock/` files become corrupted, syntactically
invalid, or otherwise unusable, **erase the entire directory** and restart the
program.  Settings will fall back to repo `config/` and `*.example` files
(or built-in defaults where no file exists):

```
    rm -rf ~/.config/pi-smart-clock
```

Alarm data, alert photos, and backups under `~/.local/share/` and
`~/.local/state/` are **not** removed by this command.  Delete
`~/.cache/pi-smart-clock/` separately if you also want to clear weather
cache and force a fresh network fetch.

**PERSISTENCE LAYOUT (EMBEDDED)**

All virtual `/sd/...` paths map to the removable SD card, accessed over
**I2C** by default.  Faster **SPI** and **SDIO** modes are planned.

**LINUX SERIAL PERMISSIONS (ESP8266)**

Before enabling the ESP8266 bridge, add your user to the `dialout` group:

```
    sudo usermod -aG dialout $USER
```

Log out and back in.  This procedure works on Debian, Ubuntu, Fedora, Arch,
and other standard Linux distributions.


---

## CHAPTER 2 — CONFIGURATION FILE FORMAT

All `*.conf` files use the same line-oriented syntax.

```
    SYNTAX RULES
    ------------

    key=value          Assign a parameter.
    # comment          Lines beginning with # are ignored.
    text # comment     Inline comments after # are stripped (esp8266, faces).
    blank line         Ignored.

    Keys are matched without regard to upper/lower case.
    Surrounding whitespace around keys and values is trimmed.
```

**BOOLEAN VALUES** (used by `enabled=` fields)

| Value | Result |
|-------|--------|
| `1`, `true`, `yes`, `on` | TRUE |
| anything else | FALSE |

**ALTERNATE KEY NAMES**

Several parameters accept synonyms.  All synonyms set the same internal
register.  See Appendix A for the complete list.


---

## CHAPTER 3 — ESP8266 SERIAL BRIDGE

**FILE:** `config/esp8266.conf`
**PURPOSE:** Route WiFi, HTTP, NTP, and MQTT through an ESP8266 coprocessor
connected to the host UART (TX/RX, 3.3 V logic).

```
    WIRING DIAGRAM (HOST ↔ ESP8266)
    -------------------------------

         HOST                         ESP8266 MODULE
         ----                         --------------
         TX  ---------------------->  RX
         RX  <----------------------  TX
         GND ---------------------->  GND

    WARNING:  Use 3.3 V logic levels only.  A 5 V host UART requires a
              level shifter.
```

Flash the bridge firmware from:

```
    firmware/esp8266/smart_clock_bridge/smart_clock_bridge.ino
```

### 3.1  PARAMETERS

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `enabled` | boolean | `false` | When TRUE, open serial port at startup and use ESP8266 for network I/O. When FALSE or port unavailable, host network is used instead. |
| `port` | string | `auto` | Serial device path. `auto` scans for USB adapters (`ttyUSB*`, `ttyACM*`) and onboard UARTs (`ttyAMA*`, `ttyS*`). Or specify e.g. `/dev/ttyUSB0` or `/dev/serial/by-id/usb-…`. |
| `baud` | integer | `115200` | UART speed in bits per second. Must match bridge firmware (`Serial.begin(115200)`). |
| `wifi_ssid` | string | *(empty)* | WiFi network name. Also accepted: `ssid`. Sent to the bridge after connect if non-empty. |
| `wifi_password` | string | *(empty)* | WiFi passphrase. Also accepted: `wifi_pass`, `password`. |

### 3.2  EXAMPLE

```
# ESP8266 WiFi bridge over serial (TX/RX, 3.3V)
enabled=true
port=auto
baud=115200
wifi_ssid=MyHomeNetwork
wifi_password=SecretPassphrase
```

### 3.3  STARTUP BEHAVIOUR

When `enabled=true`:

1. Resolve serial port (`port` or auto-detect).
2. Open port at `baud` rate.
3. Send `PING` — expect `PONG`.
4. If `wifi_ssid` is set, send `WIFI <ssid><TAB><password>`.
5. HTTP requests (weather, alert photos) prefer the bridge; on failure, fall
   back to the host `ureq` client.

Console messages:

```
    [esp8266] loaded config/esp8266.conf
    [esp8266] opened /dev/ttyUSB0 @ 115200 baud
    [esp8266] bridge online
    [esp8266] WiFi connected (192.168.1.42)
    [esp8266] not available: <reason>     ← graceful fallback
```


---

## CHAPTER 4 — CLOCK FACE SELECTION

**FILE:** `config/faces.conf`
**PURPOSE:** Select which SVG dial module is rasterized for the main clock.

### 4.1  PARAMETERS

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `face` | string | `retro-roman` | Face module identifier. Asset loaded from `assets/faces/<name>/face.svg`. |

### 4.2  RECOGNIZED FACE VALUES

| Value | Internal ID | Asset path |
|-------|-------------|------------|
| `retro-roman` | RetroRoman | `assets/faces/retro-roman/face.svg` |
| `retro_roman` | RetroRoman | *(same)* |
| `default` | RetroRoman | *(same)* |

Any other value is rejected; the program falls back to the default face.

### 4.3  EXAMPLE

```
# Clock face module (assets/faces/<name>/face.svg)
face=retro-roman
```


---

## CHAPTER 5 — BOTTOM PANEL SLOTS

**FILE:** `config/panels.conf`
**PURPOSE:** Assign feature modules to the three bottom screen panels.

```
    SCREEN LAYOUT (LOGICAL 960 × 1280, PORTRAIT)
    --------------------------------------------

    ┌─────────────────────────────────────┐
    │                                     │
    │           CLOCK FACE                │
    │                                     │
    ├───────────┬───────────┬─────────────┤
    │  b_left   │   b_mid   │   b_right   │
    │  320×300  │  320×300  │   320×300   │
    └───────────┴───────────┴─────────────┘
```

### 5.1  SLOT NAMES (LEFT SIDE OF `=`)

| Slot key | Position |
|----------|----------|
| `b_left` | Left panel |
| `b_mid` | Centre panel |
| `b_right` | Right panel |

**All three slots must be assigned.**  Omitting a slot produces a parse error
and the default layout is restored.

### 5.2  MODULE NAMES (RIGHT SIDE OF `=`)

| Module | Description |
|--------|-------------|
| `weather` | Temperature, conditions, AQI from Open-Meteo |
| `calendar` | Upcoming events (placeholder data on Linux build) |
| `holidays` | Upcoming holidays (placeholder data on Linux build) |

Each module may appear in only one slot.  Permutations are valid.

### 5.3  DEFAULT ASSIGNMENT

```
    b_left  = weather
    b_mid   = calendar
    b_right = holidays
```

### 5.4  EXAMPLE

```
# Bottom panel slot assignments (slot=module).
b_left=weather
b_mid=calendar
b_right=holidays
```


---

## CHAPTER 6 — WEATHER DATA SOURCE

**FILE:** `config/weather.conf`
**PURPOSE:** Location, units, and refresh interval for Open-Meteo API queries.
No API key is required.

Data source: [https://open-meteo.com/](https://open-meteo.com/)

### 6.1  PARAMETERS

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `latitude` | float | `39.7684` | Decimal degrees north. Synonyms: `lat`. |
| `longitude` | float | `-86.1581` | Decimal degrees east (negative = west). Synonyms: `lon`, `lng`. |
| `timezone` | string | `auto` | IANA timezone name passed to Open-Meteo, or `auto` for automatic detection. Synonyms: `tz`. |
| `units` | string | `fahrenheit` | Temperature display and API units. Synonyms: `temperature_unit`. |
| `update_interval_minutes` | integer | `30` | Normal refresh period in minutes. Synonyms: `update_minutes`, `interval`. |
| `city` | string | *(none)* | Optional display name. When set, skips reverse-geocode lookup. Synonyms: `location`. |

### 6.2  UNITS VALUES

| Input value | API parameter | Display symbol |
|-------------|---------------|----------------|
| `fahrenheit`, `f`, `farenheit` | `fahrenheit` | °F |
| `celsius`, `c`, `centigrade` | `celsius` | °C |

### 6.3  REFRESH INTERVAL NOTES

- When `update_interval_minutes` is omitted from the file, the default is
  **30 minutes**.
- During active weather alerts, the program may refresh every **5 minutes**
  regardless of this setting (internal override).

### 6.4  CITY NAME RESOLUTION

If `city` is not set:

1. Check `cache/weather_city.json` (valid for 1 hour if coordinates unchanged).
2. Otherwise query Nominatim (OpenStreetMap) reverse geocoding.
3. Store result in city cache.

If `city` is set, that string is used directly.

### 6.5  EXAMPLE

```
# Open-Meteo settings (free, no API key required)
latitude=39.7684
longitude=-86.1581
timezone=auto
units=fahrenheit
update_interval_minutes=15

# Optional city name override (skip reverse-geocode lookup)
# city=Indianapolis
```

### 6.6  HOT RELOAD

The weather module monitors `weather.conf` modification time.  Changes are
picked up automatically without restarting the program.


---

## CHAPTER 7 — ALARM SCHEDULE

**FILE:** `config/alarms.csv` (template / example only on Linux)
**FORMAT:** Comma-separated values (CSV), one alarm per row.
**STORAGE PATH (Linux):** `~/.local/share/pi-smart-clock/config/alarms.csv`
**STORAGE PATH (embedded):** `/sd/config/alarms.csv` (SD card via I2C)

### 7.1  FILE STRUCTURE

```
    ROW 0 (HEADER — REQUIRED):
    id,hour,minute,enabled,repeat,label,sound_file,video_file,snooze_minutes

    ROWS 1..N (ALARM ENTRIES):
    <fields matching header>
```

### 7.2  FIELD REFERENCE

| Field | Type | Range | Default | Description |
|-------|------|-------|---------|-------------|
| `id` | integer | 0–3 | — | Alarm slot index. Maximum **4 alarms** (IDs 0, 1, 2, 3). Rows with `id ≥ 4` are ignored. |
| `hour` | integer | 0–23 | `7` | Hour in 24-hour local time. |
| `minute` | integer | 0–59 | `0` | Minute. |
| `enabled` | boolean | true/false | `false` | When `true`, alarm is armed. |
| `repeat` | boolean | true/false | `true` | When `true`, alarm fires every day at the set time. |
| `label` | string | — | `Alarm` | Display name. Commas and quotes are escaped per CSV rules. |
| `sound_file` | path | — | `sounds/cuckoo.wav` | WAV file played in a loop until dismissed. |
| `video_file` | path | — | *(empty)* | MP4 played in centre panel during alarm (requires ffmpeg). |
| `snooze_minutes` | integer | — | `9` | Snooze duration (reserved for future UI). |

### 7.3  MEDIA PATH RESOLUTION

The loader searches for sound and video files in this order:

```
    1.  Exact path as written (if file exists)
    2.  sounds/<filename>        (for sound_file)
    3.  videos/<filename>        (for video_file)
    4.  Project root / repo manifest directory
```

Bare filenames without a `/` are assumed to live under `sounds/` or `videos/`.

### 7.4  EXAMPLE

```
id,hour,minute,enabled,repeat,label,sound_file,video_file,snooze_minutes
0,7,0,true,true,Morning,sounds/cuckoo.wav,videos/morning.mp4,9
1,12,30,false,true,Lunch,sounds/chime.wav,videos/lunch.mp4,9
```

### 7.6  LEGACY 8-COLUMN FORMAT

Older alarm files with only eight columns (no `video_file`) are still accepted.
When the eighth field parses as an integer, it is interpreted as
`snooze_minutes` and `video_file` is left empty:

```
id,hour,minute,enabled,repeat,label,sound_file,snooze_minutes
0,7,0,true,true,Morning,sounds/cuckoo.wav,9
```

### 7.7  PERSISTENCE

When alarms are saved from the menu, the program writes:

```
    Linux:    ~/.local/share/pi-smart-clock/config/alarms.csv
              ~/.local/state/pi-smart-clock/backups/alarms_YYYYMMDD_HHMMSS.csv.bak

    Embedded: /sd/config/alarms.csv
              /sd/config/alarms_YYYYMMDD_HHMMSS.csv.bak
```

On first launch, alarms are loaded from the Linux persistence path if
present; otherwise from `config/alarms.csv` or `config/alarms.csv.example`
in the repo.


---

## CHAPTER 8 — MEDIA AND ASSET PATHS

These directories are not configured via `*.conf` files but are referenced
by alarm and chime settings.

### 8.1  CHIME SOUNDS (AUTOMATIC)

| Chime event | Default path |
|-------------|--------------|
| Tick | `sounds/tick.wav` |
| Tock | `sounds/tock.wav` |
| Quarter hour | `sounds/quarter.wav` |
| Half hour | `sounds/half.wav` |
| Hour | `sounds/bell.wav` |

Place WAV files in `sounds/`.  See `sounds/README.txt`.

### 8.2  ALARM VIDEO

Place MP4 files in `videos/`.  See `videos/README.txt`.  Playback uses
**ffmpeg** on the Linux build.

### 8.3  CLOCK FACE ASSETS

```
    assets/faces/<face-name>/face.svg
    assets/fonts/DejaVuSerif-Bold.ttf     (numeral glyphs)
```

### 8.4  WEATHER ICONS

```
    assets/icons/yaru/<icon-name>.svg
```

Icons are selected automatically from WMO weather codes.


---

## CHAPTER 9 — RUNTIME CACHE FILES

These files are **written by the program**, not edited by the user.  Listed
here for completeness.

| Virtual path | Linux location | Purpose | Invalidation |
|--------------|----------------|---------|--------------|
| `cache/weather_city.json` | `~/.cache/pi-smart-clock/weather_city.json` | Cached reverse-geocode city name | Coordinate change, config mtime change, or age > 1 hour |
| `cache/weather_data.json` | `~/.cache/pi-smart-clock/weather_data.json` | Cached forecast snapshot | Coordinate/units/interval change, or age > `update_interval_minutes` |

On embedded, the same virtual paths live under `/sd/cache/` on the SD card.

Delete cache files to force a fresh network fetch.


---

## CHAPTER 10 — PI PICO 1 AND PI PICO 2 — INSTALLATION AND CONFIGURATION

This chapter covers building, flashing, and configuring the smart clock on
**Raspberry Pi Pico 1** (RP2040) and **Raspberry Pi Pico 2** (RP2350).
Linux desktop setup is in Chapters 1–9; this chapter is for the embedded
firmware path only.

### 10.1  SUPPORTED BOARDS (CURRENT TREE)

```
    BOARD                    MCU       FIRMWARE IN THIS REPO
    -----                    ---       ---------------------
    Raspberry Pi Pico 1      RP2040    YES  (default embedded target)
    Raspberry Pi Pico 1 W    RP2040    YES  (same firmware; WiFi optional)
    Raspberry Pi Pico 2      RP2350    PLANNED  (toolchain notes below)
    Raspberry Pi Pico 2 W    RP2350    PLANNED  (on-board WiFi; see §10.3)
```

The checked-in firmware (`pico-dvi` feature) is built and tested for
**Pico 1 / RP2040**.  Pico 2 support requires a different Rust compilation
target and an `embassy-rp` chip feature (`rp235xa` or `rp235xb`) — the HAL
already knows about RP2350, but this repository has not yet split the
`pico-dvi` profile for it.  Until that lands, use a **Pico 1** module on
the Pico DVI Sock.

### 10.2  PICO 1 vs PICO 2 — WHAT DIFFERS

```
┌────────────────────────┬──────────────────────────┬──────────────────────────┐
│                        │ PICO 1 (RP2040)          │ PICO 2 (RP2350)          │
├────────────────────────┼──────────────────────────┼──────────────────────────┤
│ CPU                    │ Dual Cortex-M0+ @ 133 MHz │ Dual Cortex-M33 @ 150 MHz│
│                        │ (overclockable in SW)    │ (+ optional RISC-V cores)│
├────────────────────────┼──────────────────────────┼──────────────────────────┤
│ Rust target            │ thumbv6m-none-eabi       │ thumbv8m.main-none-eabihf│
├────────────────────────┼──────────────────────────┼──────────────────────────┤
│ embassy-rp feature     │ rp2040                   │ rp235xa or rp235xb       │
├────────────────────────┼──────────────────────────┼──────────────────────────┤
│ probe-rs chip name     │ RP2040                   │ RP2350                   │
├────────────────────────┼──────────────────────────┼──────────────────────────┤
│ Flash / BOOTSEL        │ Hold BOOTSEL + USB        │ Same user procedure      │
├────────────────────────┼──────────────────────────┼──────────────────────────┤
│ Pico DVI Sock          │ Supported (800×480 DVI)  │ Header-compatible*       │
├────────────────────────┼──────────────────────────┼──────────────────────────┤
│ WiFi in this project   │ ESP8266 UART coprocessor │ Pico 2 W: on-chip WiFi   │
│                        │ (config/esp8266.conf)    │ (future; may replace ESP)│
├────────────────────────┼──────────────────────────┼──────────────────────────┤
│ Config storage         │ SD card at /sd/…         │ Same layout              │
├────────────────────────┼──────────────────────────┼──────────────────────────┤
│ Build command (today)  │ ./scripts/pico-build.sh  │ Not yet — see §10.8       │
└────────────────────────┴──────────────────────────┴──────────────────────────┘

* The Pico DVI Sock mates with the 40-pin Pico header on both generations.
  You must still flash firmware built for the correct MCU (RP2040 vs RP2350).
```

**Practical rule:** treat Pico 1 and Pico 2 as **different firmware images**.
A binary built for RP2040 will not run on RP2350, and vice versa.

### 10.3  REFERENCE HARDWARE STACK

Typical clock assembly (Pico 1, as implemented in this branch):

```
    COMPONENT              INTERFACE        NOTES
    ---------              -----------      -----
    Raspberry Pi Pico 1    —                Main MCU
    Pico DVI Sock          DVI + GPIO       800×480 display output
    DS3231 RTC module      I2C              Wall time (battery backed)
    microSD module         I2C (default)    Config, alarms, cache
    ESP8266 module         UART 3.3 V       WiFi / HTTP / NTP / MQTT
    Rotary encoder + btn   GPIO             Menu navigation
    AHT20 (optional)       I2C              Temperature / humidity
    I2S microphone (opt.)  I2S              Voice commands (future)
```

**Pico 1 W / Pico 2 W:** the wireless variants include a CYW43439 WiFi chip.
This project still uses the **ESP8266 serial bridge** on the Pico 1 path
(`config/esp8266.conf`).  On Pico 2 W, on-board WiFi will eventually remove
the external ESP8266 for network I/O, but that integration is not complete
in the current tree — plan wiring for ESP8266 unless you are on pure Pico 2
bring-up with WiFi disabled.

### 10.4  DEVELOPMENT HOST — TOOLCHAIN (BOTH BOARDS)

Embedded builds require **rustup** (not the Debian/Ubuntu `apt` `cargo` /
`rustc` packages).  Apt Rust cannot install the `thumbv6m-none-eabi` or
`thumbv8m.main-none-eabihf` cross-compilation sysroots.

```
    STEP 1 — Install rustup (once per machine)
    ------------------------------------------
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"

    STEP 2 — Ensure rustup wins over apt in PATH
    --------------------------------------------
    which cargo rustc
    # Expected: /home/<user>/.cargo/bin/cargo
    #           /home/<user>/.cargo/bin/rustc

    If you see /usr/bin/cargo, either:
        export PATH="$HOME/.cargo/bin:$PATH"
    or remove the conflict:
        sudo apt remove rustc cargo

    STEP 3 — One-time embedded target setup (Pico 1)
    ------------------------------------------------
    cd pi-smart-clock
    ./scripts/setup-embedded.sh
    # Installs: rustup target add thumbv6m-none-eabi
```

For **Pico 2** (when firmware support is added), the host will also need:

```
    rustup target add thumbv8m.main-none-eabihf
```

### 10.5  BUILD FIRMWARE (PICO 1)

```
    IMPORTANT:  linux-full is the default Cargo feature.
                 Pico builds must disable it.

    COMMAND (recommended)              RESULT
    ---------------------              ------
    ./scripts/pico-build.sh            Debug firmware ELF
    ./scripts/pico-build.sh --release  Optimised release ELF

    Equivalent manual command:
    cargo build --no-default-features --features pico-dvi \
        --target thumbv6m-none-eabi

    Output binary:
    target/thumbv6m-none-eabi/debug/pi-smart-clock-firmware
    target/thumbv6m-none-eabi/release/pi-smart-clock-firmware   (--release)
```

Do **not** run `cargo build --features pico-dvi` without
`--no-default-features` — that enables both `linux-full` and `pico-dvi`
and produces misleading `byteorder` / `nb` / `core` errors.

### 10.6  FLASH FIRMWARE ONTO THE MCU

#### 10.6.1  Pico 1 (RP2040)

**Option A — USB BOOTSEL (no debugger)**

```
    1. Hold BOOTSEL on the Pico.
    2. Plug in USB (or reset while holding BOOTSEL).
    3. The board appears as a USB mass-storage device (RPI-RP2).
    4. Copy a .uf2 file to that volume (when UF2 packaging is enabled),
       or use probe-rs / elf2uf2-rs from the release ELF (see Option B).
```

**Option B — probe-rs (SWD debugger, recommended for dev)**

```
    cargo install probe-rs-tools --locked

    probe-rs download --chip RP2040 \
        target/thumbv6m-none-eabi/release/pi-smart-clock-firmware

    probe-rs run --chip RP2040 \
        target/thumbv6m-none-eabi/release/pi-smart-clock-firmware
```

The project `.cargo/config.toml` sets `runner = "probe-run --chip RP2040"`
for the `thumbv6m-none-eabi` target (legacy alias; `probe-rs run` is the
modern equivalent).

#### 10.6.2  Pico 2 (RP2350) — when supported

Use the **same BOOTSEL workflow**, but flash an RP2350-built artifact only:

```
    probe-rs download --chip RP2350 \
        target/thumbv8m.main-none-eabihf/release/pi-smart-clock-firmware

    # UF2 volume label differs; always match chip to binary.
```

Flashing an RP2040 binary on a Pico 2 (or the reverse) will not boot.

### 10.7  SD CARD — CONFIGURATION ON DEVICE

On embedded, settings live on the removable SD card.  Virtual paths in code
map to files under the `/sd/` root (FAT volume).  Prepare the card on a PC,
then insert before power-on.

```
    SD CARD LAYOUT (FAT32)
    ----------------------

    /sd/config/esp8266.conf      WiFi bridge (UART) — copy from example
    /sd/config/faces.conf        Clock face selection
    /sd/config/panels.conf       Bottom panel module slots
    /sd/config/weather.conf      Latitude, longitude, units, update interval
    /sd/config/alarms.csv        Alarm schedule (see Chapter 7)
    /sd/cache/                   Written at runtime (weather JSON)
    /sd/alerts/                  Alert photo cache (BMP)
    /sd/sounds/                  Alarm / chime WAV files
    /sd/videos/                  Alarm MP4 files (device playback)
```

**Populate from examples on your PC:**

```
    # Mount SD card at /media/$USER/CLOCK (example)
    mkdir -p /media/$USER/CLOCK/config
    cp config/esp8266.conf.example  /media/$USER/CLOCK/config/esp8266.conf
    cp config/faces.conf.example  /media/$USER/CLOCK/config/faces.conf
    cp config/panels.conf.example /media/$USER/CLOCK/config/panels.conf
    cp config/weather.conf.example /media/$USER/CLOCK/config/weather.conf
    cp config/alarms.csv.example  /media/$USER/CLOCK/config/alarms.csv
```

Syntax for each file is identical to the Linux chapters (2–7).  There is
no `~/.config` on device — `/sd/config/` is authoritative.

**ESP8266 on device:** edit `/sd/config/esp8266.conf`.  Set `enabled=true`,
`wifi_ssid`, and `wifi_password`.  Use `port=` only if the bridge is not on
the default UART pins wired at build time (`auto` is Linux-oriented).

**Recovery:** if settings are corrupt, delete the affected files or reformat
the `config/` subtree from the `.example` sources.  Alarm backups are written
as `/sd/config/alarms_YYYYMMDD_HHMMSS.csv.bak` (Chapter 7).

### 10.8  PICO 2 — BUILD PROFILE (UPSTREAM / NOT YET IN REPO)

When Pico 2 support is merged, expect a separate Cargo feature (e.g.
`pico2-dvi`) or a `PICO_BOARD=pico2` build flag.  The differences from
today's `pico-dvi` profile will be:

```
    Cargo.toml embassy-rp feature:   rp2040  →  rp235xa  (or rp235xb)
    Rust target:                     thumbv6m-none-eabi  →  thumbv8m.main-none-eabihf
    probe-rs --chip:                 RP2040  →  RP2350
    Platform module:                 rp2040.rs  →  rp2350.rs (planned)
```

Until that profile exists, developers experimenting with Pico 2 should
track the `embassy-rp` RP2350 examples and avoid assuming Pico 1 binaries
will run.

### 10.9  RUNTIME CONSTANTS (BOTH BOARDS)

Clock overclock and network defaults for embedded are compiled into
`src/config.rs` (not read from SD):

| Constant | Default | Description |
|----------|---------|-------------|
| `CLOCK_SPEED_HZ` | `270_000_000` | System clock (270 MHz; heatsink recommended on Pico 1) |
| `VREG_VOLTAGE` | `1.20` | Core voltage for overclock |
| `FLASH_DIVIDER` | `2` | Flash clock divider |
| `MQTT_BROKER` | `192.168.1.100` | LAN broker address |
| `MQTT_PORT` | `1883` | MQTT port |

Pico 2 may use different safe overclock limits; revisit these constants when
the RP2350 platform module lands.

### 10.10  VERIFICATION CHECKLIST

```
    [ ]  which cargo → ~/.cargo/bin/cargo
    [ ]  rustup target list --installed | grep thumbv6m   (Pico 1)
    [ ]  ./scripts/pico-build.sh completes without errors
    [ ]  probe-rs lists chip (RP2040 or RP2350) when debugger attached
    [ ]  SD card mounted: /sd/config/*.conf present
    [ ]  esp8266.conf: enabled + WiFi credentials (if using UART bridge)
    [ ]  alarms.csv: at least one row with valid sound path under /sd/sounds/
    [ ]  Boot screen shows "Pico DVI + ESP8266" then clock face
```


---

## APPENDIX A — QUICK REFERENCE TABLES

### A.1  ALL CONFIGURATION FILES

| File | Required | Format |
|------|----------|--------|
| `config/esp8266.conf` | No | key=value |
| `config/faces.conf` | No | key=value |
| `config/panels.conf` | No | key=value |
| `config/weather.conf` | No | key=value |
| `config/alarms.csv` | No | CSV |

### A.2  MASTER PARAMETER LIST

```
┌─────────────────────┬──────────────────────────────────┬─────────────────────┐
│ FILE                │ KEY (SYNONYMS)                   │ DEFAULT             │
├─────────────────────┼──────────────────────────────────┼─────────────────────┤
│ esp8266.conf        │ enabled                          │ false               │
│                     │ port                             │ auto                │
│                     │ baud                             │ 115200              │
│                     │ wifi_ssid (ssid)                 │ ""                  │
│                     │ wifi_password (wifi_pass,        │ ""                  │
│                     │   password)                      │                     │
├─────────────────────┼──────────────────────────────────┼─────────────────────┤
│ faces.conf          │ face                             │ retro-roman         │
├─────────────────────┼──────────────────────────────────┼─────────────────────┤
│ panels.conf         │ b_left                           │ weather             │
│                     │ b_mid                            │ calendar            │
│                     │ b_right                          │ holidays            │
├─────────────────────┼──────────────────────────────────┼─────────────────────┤
│ weather.conf        │ latitude (lat)                   │ 39.7684             │
│                     │ longitude (lon, lng)             │ -86.1581            │
│                     │ timezone (tz)                    │ auto                │
│                     │ units (temperature_unit)         │ fahrenheit          │
│                     │ update_interval_minutes          │ 30                  │
│                     │   (update_minutes, interval)     │                     │
│                     │ city (location)                  │ *(lookup)*          │
├─────────────────────┼──────────────────────────────────┼─────────────────────┤
│ alarms.csv          │ id                               │ —                   │
│                     │ hour                             │ 7                   │
│                     │ minute                           │ 0                   │
│                     │ enabled                          │ false               │
│                     │ repeat                           │ true                │
│                     │ label                            │ Alarm               │
│                     │ sound_file                       │ sounds/cuckoo.wav   │
│                     │ video_file                       │ ""                  │
│                     │ snooze_minutes                   │ 9                   │
└─────────────────────┴──────────────────────────────────┴─────────────────────┘
```


---

## APPENDIX B — ESP8266 SERIAL COMMAND SET

Host-to-bridge commands (newline-terminated, 115200 8N1).  Documented here
because `esp8266.conf` enables this channel.

```
┌──────────────────┬────────────────────────────────────┬─────────────────────┐
│ COMMAND          │ FORMAT                             │ RESPONSE            │
├──────────────────┼────────────────────────────────────┼─────────────────────┤
│ Ping             │ PING                               │ PONG                │
│ WiFi connect     │ WIFI <ssid><TAB><password>         │ WIFI OK <ip>        │
│                  │                                    │ ERR <message>       │
│ HTTP GET         │ HTTP_GET <url>                     │ HTTP OK <code> <len>│
│                  │                                    │ <raw bytes>         │
│ NTP sync         │ NTP <server>                       │ NTP OK <epoch>      │
│ MQTT connect     │ MQTT_CONN <host> <port>            │ OK                  │
│                  │   [<user><TAB><pass>]              │ ERR <message>       │
│ MQTT publish     │ MQTT_PUB <topic> <0|1> <len>       │ OK                  │
│                  │ <binary payload>                   │                     │
│ MQTT subscribe   │ MQTT_SUB <topic>                   │ OK                  │
├──────────────────┼────────────────────────────────────┼─────────────────────┤
│ Async messages   │ LOG <text>                         │ (informational)     │
│ from bridge      │ MQTT_MSG <topic> <payload>         │ (informational)     │
└──────────────────┴────────────────────────────────────┴─────────────────────┘
```

Maximum HTTP body size on bridge: **8192 bytes**.


---

## APPENDIX C — ERROR MESSAGES AND REMEDIES

```
┌────────────────────────────────────────────┬──────────────────────────────────┐
│ CONSOLE MESSAGE                            │ REMEDY                           │
├────────────────────────────────────────────┼──────────────────────────────────┤
│ [esp8266] not available: no serial port    │ Connect USB-UART adapter; set    │
│   found                                    │ port=/dev/ttyUSB0 or fix udev/   │
│                                            │ dialout group membership.        │
├────────────────────────────────────────────┼──────────────────────────────────┤
│ [esp8266] not available: open <path>       │ Check cable, permissions, and    │
│                                            │ that no other program owns port. │
├────────────────────────────────────────────┼──────────────────────────────────┤
│ [esp8266] WiFi connect failed              │ Verify wifi_ssid and             │
│                                            │ wifi_password in esp8266.conf.   │
├────────────────────────────────────────────┼──────────────────────────────────┤
│ [panels] missing slot assignment(s)        │ Add b_left, b_mid, b_right to    │
│                                            │ panels.conf.                     │
├────────────────────────────────────────────┼──────────────────────────────────┤
│ [panels] unknown module                    │ Use weather, calendar, or        │
│                                            │ holidays only.                   │
├────────────────────────────────────────────┼──────────────────────────────────┤
│ [weather] bad latitude/longitude/units     │ Correct numeric format in        │
│                                            │ weather.conf.                    │
├────────────────────────────────────────────┼──────────────────────────────────┤
│ Config parse errors / garbled settings     │ Remove broken user overrides:    │
│ after editing ~/.config/pi-smart-clock/    │ rm -rf ~/.config/pi-smart-clock │
│                                            │ then restart (repo defaults apply)│
├────────────────────────────────────────────┼──────────────────────────────────┤
│ [faces] no config found, using default     │ Copy faces.conf.example or add   │
│                                            │ face= to faces.conf.             │
├────────────────────────────────────────────┼──────────────────────────────────┤
│ [weather] no config found, using defaults  │ Copy weather.conf.example.       │
├────────────────────────────────────────────┼──────────────────────────────────┤
│ can't find crate for `core` (embedded)     │ Install cross target:            │
│   thumbv6m-none-eabi                       │ rustup target add thumbv6m-none-eabi│
│                                            │ Use rustup cargo, not apt (Ch.10).│
├────────────────────────────────────────────┼──────────────────────────────────┤
│ byteorder / nb / critical-section errors   │ Wrong features: use              │
│   on thumbv6m-none-eabi                    │ --no-default-features --features │
│                                            │ pico-dvi, or ./scripts/pico-build.sh│
├────────────────────────────────────────────┼──────────────────────────────────┤
│ features linux-full and pico-dvi are       │ Build Pico with:                 │
│   mutually exclusive                       │ ./scripts/pico-build.sh          │
├────────────────────────────────────────────┼──────────────────────────────────┤
│ /usr/bin/cargo still used                  │ source ~/.cargo/env  or          │
│                                            │ export PATH="$HOME/.cargo/bin:$PATH"│
├────────────────────────────────────────────┼──────────────────────────────────┤
│ Pico 2 boots but no display / hang         │ Wrong firmware generation: flash │
│                                            │ RP2350 binary on Pico 2 only     │
│                                            │ (Ch.10 §10.2).                   │
└────────────────────────────────────────────┴──────────────────────────────────┘
```


---

```
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║   END OF CONFIGURATION REFERENCE MANUAL                                      ║
║                                                                              ║
║   "READY."                                                                   ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
```