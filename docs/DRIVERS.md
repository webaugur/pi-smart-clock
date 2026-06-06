# Drivers Reference

Hardware drivers and network coprocessors used by Pi Smart Clock. All drivers
sit under `src/drivers/` and are invoked through the [`Platform`](SHARED_CODE.md#platform-trait)
trait unless noted.

| Driver | Bus / link | Linux | Embedded | Status |
|--------|------------|-------|----------|--------|
| [ESP8266 bridge](#esp8266-wifi-bridge) | UART 3.3 V | Yes | Planned | **Working** (Linux) |
| [DS3231 RTC](#ds3231-real-time-clock) | I2C `0x68` | Stub | Stub | Planned |
| [AHT20](#aht20-temperature--humidity) | I2C | Stub | Stub | Planned |
| [SD storage](#sd-storage) | I2C / SPI / SDIO | Via XDG | Stub | Planned (FAT) |
| [SD audio](#sd-audio) | SD + audio out | Partial | Stub | Planned |
| [Rotary encoder](#rotary-encoder--push-button) | GPIO | SDL keys | GPIO | **Working** (Linux input) |
| [I2S microphone](#i2s-microphone--voice) | I2S | Stub | Stub | Planned |
| [NTP client](#ntp-client) | ESP8266 / network | Partial | Stub | Partial |
| [MQTT client](#mqtt-client) | ESP8266 | Partial | Stub | Partial |

Config for ESP8266: [CUSTOMIZATION.md — esp8266.conf](CUSTOMIZATION.md#configesp8266conf--wifi-serial-bridge).

---

## ESP8266 WiFi bridge

Offloads WiFi, HTTP, NTP, and MQTT to an **ESP8266** module over a serial line
protocol. Used when the host (Linux PC or Pico) has no native WiFi or you want
a single coprocessor for all network I/O.

### Architecture

```
    ┌─────────────────┐   UART 115200 8N1    ┌──────────────────┐
    │  Pi Smart Clock │ ◄──────────────────► │  ESP8266         │
    │  (host)         │   newline commands   │  smart_clock_    │
    │                 │                      │  bridge.ino      │
    └─────────────────┘                      └────────┬─────────┘
                                                      │ WiFi
                                                      ▼
                                               Internet / LAN
```

**Host side (Rust)**

| File | Role |
|------|------|
| `src/drivers/esp8266/mod.rs` | `Esp8266Client`, command worker thread |
| `src/drivers/esp8266/serial_link.rs` | Port open, auto-detect, line I/O |
| `src/drivers/esp8266/config.rs` | Load `esp8266.conf` |
| `src/platform/linux.rs` | Init client, weather HTTP, MQTT/NTP hooks |

**Firmware (Arduino)**

```
firmware/esp8266/smart_clock_bridge/smart_clock_bridge.ino
```

Libraries: `ESP8266WiFi`, `PubSubClient`, `ESP8266HTTPClient`, `NTPClient`.

### Wiring

```
    HOST (Linux UART or Pico UART)     ESP8266 MODULE
    ------------------------------     ---------------
    TX  ─────────────────────────────►  RX
    RX  ◄─────────────────────────────  TX
    GND ─────────────────────────────►  GND

    WARNING: 3.3 V logic only. Level-shift if host UART is 5 V.
```

On **Linux**, common ports: `/dev/ttyUSB0`, `/dev/ttyACM0`, onboard `ttyAMA0`.
Set `port=auto` in config or an explicit path / `by-id` symlink.

On **embedded**, UART pins are fixed at board wiring time; `port=auto` does not
apply — config supplies credentials only once the serial driver is wired.

### Configuration

File: `config/esp8266.conf` (see [CUSTOMIZATION.md](CUSTOMIZATION.md)).

| Key | Default | Description |
|-----|---------|-------------|
| `enabled` | `false` | Open serial bridge at startup |
| `port` | `auto` | Device path or auto-detect (Linux) |
| `baud` | `115200` | Must match `Serial.begin(115200)` in `.ino` |
| `wifi_ssid` | — | Sent as `WIFI` command after connect |
| `wifi_password` | — | Tab-separated with SSID on wire |

**Linux permissions:** `sudo usermod -aG dialout $USER` — [LINUX.md](LINUX.md#esp8266-serial).

### Flash bridge firmware

1. Open `firmware/esp8266/smart_clock_bridge/smart_clock_bridge.ino` in Arduino IDE
   or `arduino-cli`.
2. Board: ESP8266 (e.g. NodeMCU, Wemos D1 mini).
3. Upload. Optional: set `WIFI_SSID` / `WIFI_PASS` in the sketch for boot-time
   WiFi (host can still override via `WIFI` command from config).
4. Connect TX/RX as above; enable `esp8266.conf` on the host.

### Startup sequence (Linux)

When `enabled=true`, `LinuxPlatform::init()`:

1. Load `esp8266.conf` (XDG → repo `config/` → example).
2. Resolve `port` (`serial_link::resolve_port`).
3. Open UART, spawn **esp8266-serial** worker thread.
4. `PING` → expect `PONG`.
5. If `wifi_ssid` non-empty: `WIFI <ssid><TAB><pass>` → `WIFI OK <ip>`.

On failure the host logs `[esp8266] not available: …` and continues using
host networking (`ureq`) where implemented.

### Serial protocol

Newline-terminated commands, **115200 8N1**. Responses are one line unless
HTTP returns a binary body.

| Command | Format | Response |
|---------|--------|----------|
| Ping | `PING` | `PONG` |
| WiFi | `WIFI <ssid><TAB><password>` | `WIFI OK <ip>` or `ERR …` |
| HTTP GET | `HTTP_GET <url>` | `HTTP OK <status> <len>` then raw bytes |
| NTP | `NTP <server>` | `NTP OK <epoch>` |
| MQTT connect | `MQTT_CONN <host> <port> [<user><TAB><pass>]` | `OK` / `ERR …` |
| MQTT publish | `MQTT_PUB <topic> <0\|1> <len>` + payload bytes | `OK` |
| MQTT subscribe | `MQTT_SUB <topic>` | `OK` |

**Async lines from bridge (not command replies):**

| Prefix | Meaning |
|--------|---------|
| `LOG ` | Informational; printed to stderr |
| `MQTT_MSG ` | Incoming MQTT message |

**Limits**

| Limit | Value |
|-------|-------|
| Max HTTP body (firmware) | **8192 bytes** |
| MQTT publish buffer (firmware) | **1024 bytes** |
| Serial line (host parser) | **8192 bytes** |
| HTTP command timeout (host) | 60 s |
| WiFi connect timeout (host) | 30 s |

### Rust API (`Esp8266Client`)

Linux-only module (`#[cfg(feature = "linux-full")]`).

```rust
Esp8266Client::open(&cfg)?;   // open port, ping, optional WiFi
client.ping()?;
client.wifi_connect(ssid, pass)?;
client.http_get(url)?;          // Vec<u8>
client.ntp(server)?;            // RFC3339 or epoch string
client.mqtt_connect(broker, port, user, pass)?;
client.mqtt_publish(topic, payload, retain)?;
client.mqtt_subscribe(topic)?;
```

Commands are queued to a background thread; callers block with per-command
timeouts. `Clone` on the client shares the same worker channel.

### Integration points

| Feature | How ESP8266 is used |
|---------|---------------------|
| Weather fetch | `LinuxPlatform::fetch_weather` → `http_get` Open-Meteo URL; fallback `ureq` |
| Binary download | `http_download_binary` → ESP8266 first, then `ureq` |
| NTP | `NtpClient::sync` → `platform.esp8266_get_ntp("pool.ntp.org")` |
| MQTT | `MqttClient` → `Platform::esp8266_mqtt_*` |

Embedded `Platform` implementations return no-op / `None` for these methods
until UART bridge code is ported to `pico-dvi`.

### Troubleshooting

| Symptom | Check |
|---------|--------|
| `no serial port found` | Cable, `port=`, USB adapter, `dialout` group |
| `open /dev/…` permission denied | `dialout`, unplug other serial monitors |
| `expected PONG` | Baud rate, TX/RX swapped, wrong firmware |
| `WiFi connect failed` | SSID/password, 2.4 GHz network |
| `HTTP status` not 200 | URL, TLS (ESP8266 HTTP is plain) |
| `http body too large` | Response > 8 KB — use host `ureq` fallback on Linux |
| `esp8266 command timeout` | Bridge hung; reset ESP8266 |

---

## DS3231 real-time clock

**Purpose:** Battery-backed wall clock; sync system time from hardware RTC.

| Item | Value |
|------|-------|
| I2C address | `0x68` (`DS3231::ADDR`) |
| Source | `src/drivers/ds3231.rs` |

```rust
DS3231::synchronize(platform).await;  // read RTC → set system time
DS3231::set_time(platform, hour, minute).await;
```

**Status:** API stub — I2C read/write not implemented. Used by
`clock_core` time-set flow once wired on Pico.

---

## AHT20 temperature & humidity

**Purpose:** Environmental sensor for status bar and weather context.

| Item | Value |
|------|-------|
| Source | `src/drivers/aht20.rs` |
| Return | `(temp_c, humidity)` |

```rust
let (t, h) = Aht20Sensor::read(platform).await;
```

**Status:** Returns placeholder `(23.7, 51.2)`. Full I2C + CRC sequence TODO.
Called from `clock_core::sensors::EnvSensor::read`.

---

## SD storage

**Purpose:** Removable FAT volume at `/sd/` for config, alarms, cache, media on
embedded builds.

| Item | Value |
|------|-------|
| Source | `src/drivers/sd_storage.rs` |
| Default bus | **I2C** (`StorageBusMode::I2c`) |
| Planned | SPI, SDIO |

```rust
let mut sd = SdStorage::new(StorageBusMode::I2c);
sd.mount()?;
sd.read_file("/sd/config/alarms.csv")?;
sd.write_file(path, &data)?;
```

**Status:** `mount()` succeeds as stub; read/write return "not yet implemented".
Wired from `PicoDviPlatform` for `Platform::read_file` / `write_file`.

Linux uses XDG paths instead — [LINUX.md](LINUX.md#persistence).

---

## SD audio

**Purpose:** Play alarm/chime WAV from SD without loading entire file into RAM
on embedded.

| Source | `src/drivers/sd_audio.rs` |

```rust
play_wav_from_sd(platform, "sounds/alarm.wav").await;
```

**Status:** Delegates to `platform.play_raw_audio`. Linux logs path; streaming
from SD over I2C not implemented.

---

## Rotary encoder & push button

**Purpose:** Menu navigation (encoder) and confirm/dismiss (button).

| Source | `src/drivers/rotary_encoder.rs` |

```rust
encoder.update(platform).await;
// encoder.value, encoder.button_pressed
```

**Linux:** SDL arrow keys → `read_rotary_delta`; Space → `read_pushbutton`
(`platform/linux.rs`).

**Embedded:** GPIO implementation planned on `PicoDviPlatform`.

---

## I2S microphone & voice

**Purpose:** Wake-word / voice commands (future).

| Source | `src/drivers/microphone.rs` |

```rust
voice.listen(platform).await;
// voice.wake_word_detected, voice.last_energy
```

Reads `platform.read_i2s_samples(512)`; energy threshold stub (`> 50_000`).
Pairs with `clock_core::voice_commands` (Linux-gated).

---

## NTP client

**Purpose:** Set wall clock from network time.

| Source | `src/drivers/ntp.rs` |

```rust
NtpClient::sync(platform).await?;
```

Uses `platform.esp8266_get_ntp("pool.ntp.org")` when bridge is up. On Linux,
parses RFC3339 from bridge response. On embedded, accepts non-empty string
(stub). Falls back to error if bridge unavailable.

---

## MQTT client

**Purpose:** Home Assistant / LAN telemetry (broker from `src/config.rs`).

| Source | `src/drivers/mqtt.rs` |
| Default broker | `192.168.1.100:1883` (compile-time) |

```rust
let mut mqtt = MqttClient::new();
mqtt.connect(platform, broker, port).await;
mqtt.publish(platform, "smart-clock/status", "online").await;
mqtt.subscribe(platform, "smart-clock/cmd").await;
```

All traffic goes through ESP8266 `MQTT_*` commands when bridge is connected.
Credentials: optional third-party tab-separated user/pass on `MQTT_CONN`.

---

## Adding a new driver

1. Implement hardware access in `src/drivers/<name>.rs`.
2. Add methods to `Platform` in `src/drivers/platform.rs` if the rest of the
   app needs it.
3. Implement on `LinuxPlatform` and `PicoDviPlatform`.
4. Export from `src/drivers/mod.rs` with correct `#[cfg(feature = …)]`.
5. Document wiring, config, and status here.

Keep blocking UART/I2C work off async executors — follow ESP8266 pattern
(`spawn_blocking` on Linux, dedicated task on Embassy for Pico).

---

## Related docs

| Doc | Topic |
|-----|-------|
| [CUSTOMIZATION.md](CUSTOMIZATION.md) | `esp8266.conf` and other user settings |
| [LINUX.md](LINUX.md) | Serial permissions, host network fallback |
| [EMBEDDED.md](EMBEDDED.md) | Pico wiring, SD card, future UART bridge |
| [SHARED_CODE.md](SHARED_CODE.md) | `Platform` trait and module map |