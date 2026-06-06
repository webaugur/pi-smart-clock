#!/usr/bin/env bash
# Pico cross-build — forces rustup's cargo/rustc ahead of apt's /usr/bin.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ -f "${HOME}/.cargo/env" ]]; then
  # shellcheck source=/dev/null
  source "${HOME}/.cargo/env"
else
  export PATH="${HOME}/.cargo/bin:${PATH}"
fi

if ! command -v rustup >/dev/null 2>&1; then
  echo "error: rustup not found. Install from https://rustup.rs" >&2
  echo "       apt's cargo/rustc cannot build Pico firmware." >&2
  exit 1
fi

if [[ "$(command -v cargo)" == /usr/bin/cargo ]]; then
  echo "error: still using apt cargo at /usr/bin/cargo" >&2
  echo "       run:  source \"\${HOME}/.cargo/env\"" >&2
  echo "       or:   export PATH=\"\${HOME}/.cargo/bin:\${PATH}\"" >&2
  exit 1
fi

rustup target add thumbv6m-none-eabi >/dev/null 2>&1 || true

exec cargo build --no-default-features --features pico-dvi --target thumbv6m-none-eabi "$@"