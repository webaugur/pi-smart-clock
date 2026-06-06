#!/usr/bin/env bash
# Build dependencies for Debian source packages (Trixie).
set -euo pipefail

if [[ "${EUID}" -eq 0 ]]; then
  APT=(apt-get)
else
  APT=(sudo apt-get)
fi

"${APT[@]}" update
"${APT[@]}" install -y --no-install-recommends \
  build-essential \
  debhelper \
  devscripts \
  lintian \
  fakeroot \
  cargo \
  rustc \
  git \
  pkg-config \
  libssl-dev \
  libsdl2-dev \
  libsdl2-ttf-dev \
  libsdl2-mixer-dev \
  fonts-dejavu-core \
  ffmpeg

echo "==> Debian packaging build dependencies installed."