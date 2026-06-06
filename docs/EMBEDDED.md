# Embedded Guide — Pi Pico 1 & 2

Firmware for Raspberry Pi Pico + Pico DVI Sock. Cargo feature: `pico-dvi` (requires `--no-default-features`).

Config on SD card: [CUSTOMIZATION.md](CUSTOMIZATION.md).  
Architecture: [SHARED_CODE.md](SHARED_CODE.md).

---

## Supported boards (current tree)

```
    BOARD                    MCU       FIRMWARE IN THIS REPO
    -----                    ---       ---------------------
    Raspberry Pi Pico 1      RP2040    YES
    Raspberry Pi Pico 1 W    RP2040    YES (same image; ESP8266 optional)
    Raspberry Pi Pico 2      RP2350    PLANNED
    Raspberry Pi Pico 2 W    RP2350    PLANNED (on-chip WiFi later)
```

Checked-in firmware targets **Pico 1 / RP2040** (`thumbv6m-none-eabi`, `embassy-rp` feature `rp2040`). Use a Pico 1 module on the DVI Sock until Pico 2 profile lands.

---

## Pico 1 vs Pico 2

```
┌────────────────────────┬──────────────────────────┬──────────────────────────┐
│                        │ PICO 1 (RP2040)          │ PICO 2 (RP2350)          │
├────────────────────────┼──────────────────────────┼──────────────────────────┤
│ CPU                    │ Dual Cortex-M0+            │ Dual Cortex-M33          │
│ Rust target            │ thumbv6m-none-eabi       │ thumbv8m.main-none-eabihf│
│ embassy-rp feature     │ rp2040                   │ rp235xa / rp235xb        │
│ probe-rs chip          │ RP2040                   │ RP2350                   │
│ Pico DVI Sock          │ Supported                │ Header-compatible*       │
│ WiFi (this project)    │ ESP8266 UART             │ Pico 2 W: on-chip (TBD)  │
│ Config storage         │ SD card /sd/…            │ Same layout              │
│ Build today            │ ./scripts/pico-build.sh  │ Not yet — §Pico 2 below  │
└────────────────────────┴──────────────────────────┴──────────────────────────┘

* Same 40-pin header; firmware must match MCU generation.
```

**Rule:** RP2040 and RP2350 binaries are not interchangeable.

---

## Reference hardware (Pico 1)

```
    COMPONENT              INTERFACE        NOTES
    ---------              -----------      -----
    Raspberry Pi Pico 1    —                Main MCU
    Pico DVI Sock          DVI + GPIO       800×480
    DS3231 RTC             I2C              Wall time
    microSD module         I2C (default)    Config / alarms / cache
    ESP8266                UART 3.3 V       WiFi (esp8266.conf)
    Rotary encoder + btn   GPIO             Menu
    AHT20 (optional)       I2C              Temp / humidity
    I2S mic (optional)     I2S              Voice (future)
```

Pico W / Pico 2 W include CYW43439 WiFi; this tree still documents ESP8266 for network bring-up on Pico 1.

Per-driver wiring, protocol, and implementation status: **[DRIVERS.md](DRIVERS.md)**.

---

## Development host toolchain

Embedded builds require **rustup** — apt `cargo`/`rustc` cannot install `thumbv6m-none-eabi` sysroot.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

which cargo rustc
# Expected: ~/.cargo/bin/cargo  (NOT /usr/bin/cargo)

cd pi-smart-clock
./scripts/setup-embedded.sh
```

**PATH conflict:** if `which cargo` shows `/usr/bin/cargo`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
# or: sudo apt remove rustc cargo
```

**Recommended build wrapper** (sets PATH automatically):

```bash
./scripts/pico-build.sh
./scripts/pico-build.sh --release
```

### Pico 2 (when supported)

```bash
rustup target add thumbv8m.main-none-eabihf
```

---

## Build firmware (Pico 1)

```
IMPORTANT:  default feature is linux-full — disable for Pico.

./scripts/pico-build.sh              # debug
./scripts/pico-build.sh --release    # release

# Equivalent:
cargo build --no-default-features --features pico-dvi \
    --target thumbv6m-none-eabi
```

**Output:**

```
target/thumbv6m-none-eabi/debug/pi-smart-clock-firmware
target/thumbv6m-none-eabi/release/pi-smart-clock-firmware
```

Do **not** run `cargo build --features pico-dvi` without `--no-default-features`.

---

## Flash firmware

### Pico 1 — BOOTSEL (no debugger)

1. Hold **BOOTSEL**, plug USB (or reset while holding).
2. Mount **RPI-RP2** volume.
3. Copy UF2 (when packaging enabled) or convert ELF with `elf2uf2-rs`.

### Pico 1 — probe-rs (development)

```bash
cargo install probe-rs-tools --locked

probe-rs download --chip RP2040 \
    target/thumbv6m-none-eabi/release/pi-smart-clock-firmware

probe-rs run --chip RP2040 \
    target/thumbv6m-none-eabi/release/pi-smart-clock-firmware
```

`.cargo/config.toml` sets `runner = "probe-run --chip RP2040"` for `thumbv6m-none-eabi`.

### Pico 2 — when supported

```bash
probe-rs download --chip RP2350 \
    target/thumbv8m.main-none-eabihf/release/pi-smart-clock-firmware
```

---

## SD card layout

FAT32 volume; virtual paths map to `/sd/` root (I2C SD access by default).

```
/sd/config/esp8266.conf
/sd/config/faces.conf
/sd/config/panels.conf
/sd/config/weather.conf
/sd/config/alarms.csv
/sd/cache/                 # runtime weather JSON
/sd/alerts/                # alert photo BMP cache
/sd/sounds/                # alarm / chime WAV
/sd/videos/                # alarm MP4
```

**Populate on PC:**

```bash
mkdir -p /media/$USER/CLOCK/config
cp config/*.example /media/$USER/CLOCK/config/
# rename .example → live names (see CUSTOMIZATION.md)
cp config/alarms.csv.example /media/$USER/CLOCK/config/alarms.csv
```

On device there is no `~/.config` — `/sd/config/` is authoritative. File syntax: [CUSTOMIZATION.md](CUSTOMIZATION.md).

**ESP8266 on device:** `enabled=true`, `wifi_ssid`, `wifi_password` in `/sd/config/esp8266.conf`. `port=auto` is Linux-oriented; embedded uses fixed UART wiring.

**Recovery:** replace corrupt files from `.example` templates; alarm backups at `/sd/config/alarms_YYYYMMDD_HHMMSS.csv.bak`.

---

## Pico 2 build profile (planned)

Expected changes when RP2350 support merges:

```
embassy-rp feature:  rp2040 → rp235xa (or rp235xb)
Rust target:         thumbv6m-none-eabi → thumbv8m.main-none-eabihf
probe-rs chip:       RP2040 → RP2350
Platform module:     rp2040.rs → rp2350.rs
```

Track `embassy-rp` RP2350 examples until this repo adds a `pico2-dvi` (or similar) feature.

---

## Compiled constants (not on SD)

From `src/config.rs` — change requires rebuild:

| Constant | Default | Description |
|----------|---------|-------------|
| `CLOCK_SPEED_HZ` | `270_000_000` | CPU clock (heatsink recommended) |
| `VREG_VOLTAGE` | `1.20` | Core voltage |
| `FLASH_DIVIDER` | `2` | Flash divider |
| `MQTT_BROKER` | `192.168.1.100` | MQTT host |
| `MQTT_PORT` | `1883` | MQTT port |

---

## Verification checklist

```
[ ]  which cargo → ~/.cargo/bin/cargo
[ ]  rustup target list --installed | grep thumbv6m
[ ]  ./scripts/pico-build.sh succeeds
[ ]  probe-rs sees RP2040 (if using debugger)
[ ]  SD: /sd/config/*.conf present
[ ]  esp8266.conf: WiFi credentials if using bridge
[ ]  alarms.csv: valid sound under /sd/sounds/
[ ]  Boot → clock face
```

---

## Troubleshooting

| Error | Remedy |
|-------|--------|
| `can't find crate for core` | `rustup target add thumbv6m-none-eabi`; use rustup `cargo` |
| `byteorder` / `nb` errors | `--no-default-features --features pico-dvi` or `./scripts/pico-build.sh` |
| `linux-full` and `pico-dvi` mutually exclusive | Use `./scripts/pico-build.sh` only |
| `/usr/bin/cargo` | `source ~/.cargo/env` |
| Pico 2 no boot / hang | Wrong firmware generation — flash RP2350 build only |
| SD read not implemented | FAT/I2C driver still stubbed — config load may fail until driver lands |

Config parse errors: [CUSTOMIZATION.md](CUSTOMIZATION.md#config-errors-and-remedies).