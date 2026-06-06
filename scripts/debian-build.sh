#!/usr/bin/env bash
# Build a Debian binary package and run lintian (pedantic, errors fail).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
PARENT="$(dirname "$ROOT")"

if ! command -v dpkg-buildpackage >/dev/null 2>&1; then
  echo "error: dpkg-buildpackage not found. Run: ./scripts/debian-deps.sh" >&2
  exit 1
fi

export PATH="${HOME}/.cargo/bin:${PATH}"
export DEB_BUILD_OPTIONS="${DEB_BUILD_OPTIONS:-nocheck}"

dpkg-buildpackage -uc -us -b

DEB="$(ls -1t "${PARENT}"/pi-smart-clock_*.deb | head -1)"
CHANGES="$(ls -1t "${PARENT}"/pi-smart-clock_*.changes | head -1)"
echo "==> Running lintian (pedantic, fail on error)"

run_lintian() {
  lintian --pedantic --display-info --fail-on error "$@"
}

if command -v docker >/dev/null 2>&1; then
  # Validate .changes (trixie distribution) and .deb on Debian Trixie.
  docker run --rm \
    -v "${PARENT}:/work" \
    -w /work \
    debian:trixie-slim \
    bash -lc "apt-get update -qq && apt-get install -y -qq lintian >/dev/null && lintian --pedantic --display-info --fail-on error $(basename "$CHANGES") $(basename "$DEB")"
else
  # Host lintian may not know the trixie suite in .changes; the .deb is authoritative.
  echo "warning: docker not available; lintian validates .deb only (not .changes distribution)" >&2
  run_lintian "${DEB}"
fi

echo "==> Package built:"
ls -1 "${PARENT}"/pi-smart-clock_*.deb

"${ROOT}/scripts/debian-clean.sh"