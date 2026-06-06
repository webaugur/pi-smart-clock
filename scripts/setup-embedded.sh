#!/usr/bin/env bash
# One-time setup for Pico / RP2040 cross-builds.
set -euo pipefail

RUSTUP_BIN="${HOME}/.cargo/bin/rustup"
CARGO_BIN="${HOME}/.cargo/bin/cargo"

if [[ -x "${HOME}/.cargo/bin/cargo" ]]; then
  CARGO_BIN="${HOME}/.cargo/bin/cargo"
fi

echo "==> Checking Rust toolchain"
if [[ -x "$RUSTUP_BIN" ]]; then
  echo "    rustup: $($RUSTUP_BIN --version)"
  echo "    cargo:  $($CARGO_BIN --version)"
  echo "    rustc:  $($HOME/.cargo/bin/rustc --version)"
else
  echo "error: rustup not found at ${HOME}/.cargo/bin/rustup" >&2
  echo "Install from https://rustup.rs — apt's cargo/rustc cannot install embedded targets." >&2
  exit 1
fi

if command -v /usr/bin/rustc >/dev/null && [[ "$(command -v rustc)" == "/usr/bin/rustc" ]]; then
  echo ""
  echo "warning: /usr/bin/rustc is first in PATH (apt package)."
  echo "         Pico builds need rustup's toolchain. Run:"
  echo "           export PATH=\"\${HOME}/.cargo/bin:\$PATH\""
  echo "         Or remove apt packages: sudo apt remove rustc cargo"
  echo ""
fi

echo "==> Installing thumbv6m-none-eabi target"
"$RUSTUP_BIN" target add thumbv6m-none-eabi

echo "==> Verifying embedded sysroot"
"${HOME}/.cargo/bin/rustc" --print sysroot --target thumbv6m-none-eabi >/dev/null

echo "==> Done. Build firmware with:"
echo "    export PATH=\"\${HOME}/.cargo/bin:\$PATH\""
echo "    cargo pico"