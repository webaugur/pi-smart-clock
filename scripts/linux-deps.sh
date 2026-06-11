#!/usr/bin/env bash
# Native build dependencies for the desktop target on:
# - Debian 13 (Trixie) / Raspberry Pi OS Trixie (apt)
# - OpenIndiana 2025 (pkgsrc / pkgin)
#
# Run the appropriate commands for your OS. The project no longer has a separate
# embedded/Pico target.
#
# Architectures: Debian Trixie arm64 + amd64 (packages are the same);
# OpenIndiana 2025 amd64 primary (arm64 when OI pkgsrc supports it).
set -euo pipefail

if [[ -f /etc/debian_version ]]; then
  ver="$(tr -d '[:space:]' </etc/debian_version)"
  case "$ver" in
    13.*|13|trixie|trixie/sid) ;;
    *)
      echo "warning: Linux builds target Debian 13 (Trixie); this host reports: ${ver}" >&2
      ;;
  esac
fi

if [[ "${EUID}" -eq 0 ]]; then
  APT=(apt-get)
else
  APT=(sudo apt-get)
fi

"${APT[@]}" update
"${APT[@]}" install -y --no-install-recommends \
  build-essential \
  pkg-config \
  ca-certificates \
  curl \
  libsdl2-dev \
  libsdl2-ttf-dev \
  libsdl2-mixer-dev \
  fonts-dejavu-core \
  ffmpeg

# For full Unicode / Japanese / CJK support in the UI (bottom panels, menus, etc.),
# including holiday names when country=JP etc., install a CJK font package:
#   sudo apt install fonts-noto-cjk
# or the lighter
#   sudo apt install fonts-ipafont-gothic
# The clock will automatically prefer a capable font if one is present on the system
# (see font loading in src/main.rs). DejaVu alone has no CJK glyphs.

echo "==> Trixie Linux dependencies installed."