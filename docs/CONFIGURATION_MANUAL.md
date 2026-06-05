
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

**FILE SEARCH ORDER**

The program looks for each configuration type in this order:

```
    1.  config/<name>.conf     (user file — preferred)
    2.  config/<name>.conf.example
```

If neither file exists, built-in defaults are used and a message is printed
to the console (stderr).

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

**FILE:** `config/alarms.csv`
**FORMAT:** Comma-separated values (CSV), one alarm per row.
**STORAGE PATH (Linux):** `config/alarms.csv`
**STORAGE PATH (embedded):** `/sd/config/alarms.csv`

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

When alarms are saved from the menu (embedded path), the program writes:

```
    /sd/config/alarms.csv
    /sd/config/alarms_YYYYMMDD_HHMMSS.csv.bak
```

On Linux, in-memory file storage is used until full persistence is wired.


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

| File | Purpose | Invalidation |
|------|---------|--------------|
| `cache/weather_city.json` | Cached reverse-geocode city name | Coordinate change, config mtime change, or age > 1 hour |
| `cache/weather_data.json` | Cached forecast snapshot | Coordinate/units/interval change, or age > `update_interval_minutes` |

Delete cache files to force a fresh network fetch.


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
│ [faces] no config found, using default     │ Copy faces.conf.example or add   │
│                                            │ face= to faces.conf.             │
├────────────────────────────────────────────┼──────────────────────────────────┤
│ [weather] no config found, using defaults  │ Copy weather.conf.example.       │
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