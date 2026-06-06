#!/usr/bin/env bash
# Linux desktop build (Debian 13 Trixie / Raspberry Pi OS Trixie).
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

exec cargo build --features linux-full "$@"