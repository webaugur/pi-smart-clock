#!/usr/bin/env bash
# Remove ephemeral Debian packaging outputs (keeps debian/ source metadata).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ -f debian/rules ]]; then
  debian/rules clean >/dev/null 2>&1 || true
fi

rm -rf debian/.debhelper debian/cargo-home debian/pi-smart-clock debian/tmp
rm -f debian/files debian/debhelper-build-stamp debian/*.substvars debian/*.debhelper.log

echo "==> Debian build tree cleaned."