#!/usr/bin/env bash
# Desktop build for Debian Trixie and OpenIndiana 2025 (and other Unix with SDL2).
# Uses the "full" feature (the only desktop target after Pico removal).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ -f "${HOME}/.cargo/env" ]]; then
  # shellcheck source=/dev/null
  source "${HOME}/.cargo/env"
else
  export PATH="${HOME}/.cargo/bin:${PATH}"
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo not found. Install rustup: https://rustup.rs" >&2
  exit 1
fi

if [[ "$(command -v cargo)" == /usr/bin/cargo ]]; then
  echo "error: still using apt cargo at /usr/bin/cargo" >&2
  echo "       run:  source \"\${HOME}/.cargo/env\"" >&2
  echo "       or:   export PATH=\"\${HOME}/.cargo/bin:\${PATH}\"" >&2
  exit 1
fi

ARCH=$(dpkg --print-architecture 2>/dev/null || uname -m)
echo "==> Building for architecture: ${ARCH} (Debian Trixie arm64/amd64 and OI 2025 amd64 supported)"

exec cargo build --features full "$@"