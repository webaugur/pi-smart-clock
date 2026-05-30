# 🕰️ Pi Smart Clock - Black Forest Edition

A beautiful fullscreen Raspberry Pi clock with Roman numerals, mechanical tick-tock, Black Forest chimes, Google Calendar, weather, and holidays.

## Features
- Roman numeral analog clock with bouncing second hand
- Black Forest cuckoo chimes (:15, :30, :45, hourly tolls)
- Modular bottom panels (Calendar / Holidays / Weather)
- Tokio async runtime ready for APIs
- Crontab-style chime scheduling

## Build Instructions for Raspberry Pi

### 1. Setup Raspberry Pi OS Lite
```bash
sudo apt update
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-mixer-dev libasound2-dev curl git -y
```

### 2. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 3. Clone & Build
```bash
git clone https://github.com/webaugur/pi-smart-clock.git
cd pi-smart-clock

# Create sounds folder
mkdir -p sounds
# Copy your .wav files here: tick.wav, tock.wav, cuckoo.wav, etc.

cargo build --release
```

### 4. Run
```bash
cargo run --release
```

**For fullscreen on framebuffer (no X11):**
Add to `/boot/firmware/config.txt`:
```
dtoverlay=vc4-fkms-v3d
```

Then run with:
```bash
sudo ./target/release/pi-smart-clock
```

## Project Structure
- `src/clock.rs` — Main clock face
- `src/chimes.rs` — Modular Black Forest + cron chimes
- `src/modules/` — Calendar, Holidays, Weather panels
- `src/async_runtime.rs` — Tokio background tasks

Made with ❤️ for the Raspberry Pi.

