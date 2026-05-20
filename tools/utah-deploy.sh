#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE="$ROOT/core"
cd "$CORE"

echo "[UTAH-DEPLOY] Production forge (core/)..."

cargo build --release

if cargo bootimage --release 2>/dev/null; then
  :
elif command -v bootimage >/dev/null 2>&1; then
  bootimage build --release
else
  echo "[FATAL] Install bootimage: cargo install bootimage"
  exit 1
fi

KERNEL_BIN="$CORE/target/x86_64-unknown-none/release/bootimage-utah-kernel.bin"
OUTPUT_PKG="$ROOT/utah_v1_signed.pkg"

if [[ ! -f "$KERNEL_BIN" ]]; then
  echo "[FATAL] Missing $KERNEL_BIN"
  exit 1
fi

if command -v openssl >/dev/null 2>&1; then
  openssl enc -aes-256-cbc -salt -pbkdf2 -in "$KERNEL_BIN" -out "$OUTPUT_PKG"
  echo "[SUCCESS] $KERNEL_BIN"
  echo "[SUCCESS] $OUTPUT_PKG"
else
  cp "$KERNEL_BIN" "$ROOT/utah_v1_release.bin"
  echo "[SUCCESS] $KERNEL_BIN (openssl not found — unencrypted copy at repo root)"
fi

echo "Emulate: cd core && cargo run --release"
