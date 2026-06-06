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
    Pico DVI Sock          DVI + GPIO       640×480 VGA*
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

## DVI output — direct Pico-to-cable wiring (no Sock)

Firmware uses the standard **RP2040 DVI pinout** from [PicoDVI](https://github.com/Wren6991/PicoDVI) /
[Pico DVI Sock](https://github.com/Wren6991/Pico-DVI-Sock) (same GPIO map as the Adafruit DVI Sock).
You can wire a **DVI-D** plug or cable **directly** to the Pico instead of using a Sock or HDMI
adapter board.

### Signal map

Eight GPIO pins drive the TMDS lanes. Place a **270 Ω** series resistor on **each** signal wire
(between Pico GPIO and the DVI conductor), matching the Sock schematic.

```
    PICO GPIO    PICO PIN*   TMDS LANE     DVI-D PIN†    NOTES
    ---------    ---------   ---------     ---------     -----
    GP12         16          Data 0 +      8             270 Ω in series
    GP13         17          Data 0 −      7             270 Ω in series
    GP14         19          Clock +       10            270 Ω in series
    GP15         20          Clock −       12            270 Ω in series
    GP16         21          Data 2 +      2             270 Ω in series
    GP17         22          Data 2 −      1             270 Ω in series
    GP18         24          Data 1 +      5             270 Ω in series
    GP19         25          Data 1 −      4             270 Ω in series

    GND          18, 23, …   —             3, 6, 9, 11   TMDS shield / return pins
    GND          —           Shell         DVI shell     Cable braid / connector shell
```

\* Physical pin numbers on the **40-pin Pico header** (USB connector at top, pins numbered down each
long edge per the [Pico pinout](https://datasheets.raspberrypi.com/pico/Pico-R3-A4-Pinout.pdf)).

† **DVI-D** connector pin numbers (single-link TMDS). Pin 1 is the square pad on a male DVI plug.

### Wiring sketch

```
    Raspberry Pi Pico (USB end)                DVI-D cable / plug
    ───────────────────────────                ────────────────────

    GP12 (pin 16) ──[270Ω]──────────────────►  pin 8  (TMDS Data 0 +)
    GP13 (pin 17) ──[270Ω]──────────────────►  pin 7  (TMDS Data 0 −)
    GP14 (pin 19) ──[270Ω]──────────────────►  pin 10 (TMDS Clock +)
    GP15 (pin 20) ──[270Ω]──────────────────►  pin 12 (TMDS Clock −)
    GP16 (pin 21) ──[270Ω]──────────────────►  pin 2  (TMDS Data 2 +)
    GP17 (pin 22) ──[270Ω]──────────────────►  pin 1  (TMDS Data 2 −)
    GP18 (pin 24) ──[270Ω]──────────────────►  pin 5  (TMDS Data 1 +)
    GP19 (pin 25) ──[270Ω]──────────────────►  pin 4  (TMDS Data 1 −)

    GND  (pin 18) ─────────────────────────►  pin 3  (Data 2 shield)
    GND  (pin 23) ─────────────────────────►  pin 6  (Data 1 shield)
    GND            ─────────────────────────►  pin 9  (Data 0 shield)
    GND            ─────────────────────────►  pin 11 (Clock shield)
    GND            ─────────────────────────►  shell  (connector braid)
```

### Optional pins (if the display does not sync)

Most monitors work with only the TMDS pairs and grounds above. If a panel stays blank, try:

| DVI-D pin | Signal | Suggested hook-up |
|-----------|--------|-------------------|
| 14 | +5 V (EDID / sink power) | **+5 V** from a regulated supply (not 3.3 V) |
| 15 | Hot-plug detect (HPD) | **+5 V** through a **1 kΩ** pull-up, or tie to pin 14 |

The Sock board leaves +5 V unconnected by default; many HDMI/DVI monitors still lock without it.

### Firmware video mode

Current firmware uses **640×480 @ 60 Hz (VGA timing)** via vendored [`pico-dvi-rs`](../../third_party/pico-dvi-rs) (`third_party/pico-dvi-rs`, branch `cursed-library` base). The CPU overclocks to **252 MHz** for the DVI bit clock. Logical UI layout is `Layout::dvi_vga()` (640×480); the clock face is drawn with display-list scanout in `src/platform/dvi_gfx.rs`.

Pico DVI Sock pinout (matches PicoDVI `pico_sock_cfg`):

| Lane | GPIO + / − |
|------|------------|
| Data 0 (blue) | GP12 / GP13 |
| Data 1 (green) | GP18 / GP19 |
| Data 2 (red) | GP16 / GP17 |
| Clock | GP14 / GP15 |

### Build notes

- Firmware heap is **128 KiB** (`firmware/alloc.rs`) for display-list allocation.
- Linker script `memory.x` provides scratch RAM sections required by the DVI renderer.
- Keep leads **short** and twisted as differential pairs (+/− per lane) where possible.
- Do **not** route these GPIOs through a breadboard — parasitic capacitance will corrupt the signal.
- **HDMI** cables use a different **pin numbering** than DVI-D, but the same TMDS lanes electrically;
  use a passive DVI→HDMI cable or plug if the monitor only has HDMI.
- Reference schematic: [Pico DVI Sock PDF](https://github.com/Wren6991/Pico-DVI-Sock/blob/master/dvi-sock.pdf).

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