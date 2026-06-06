#!/usr/bin/env bash
# Native build dependencies for Debian 13 (Trixie) / Raspberry Pi OS Trixie.
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

echo "==> Trixie Linux dependencies installed."