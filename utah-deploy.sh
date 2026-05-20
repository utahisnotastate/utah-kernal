#!/usr/bin/env bash
# Utah-Kernel Monolithic Forge Script — release build and optional distribution packaging.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

echo "[UTAH-DEPLOY] Starting production forge..."

if ! command -v cargo >/dev/null 2>&1; then
  echo "[FATAL] cargo not found. Install Rust: https://rustup.rs/"
  exit 1
fi

echo "[UTAH-DEPLOY] Building release kernel (x86_64-unknown-none)..."
cargo build --release

if cargo bootimage --release 2>/dev/null; then
  :
elif command -v bootimage >/dev/null 2>&1; then
  bootimage build --release
else
  echo "[FATAL] bootimage not found. Install: cargo install bootimage"
  exit 1
fi

KERNEL_BIN="$ROOT/target/x86_64-unknown-none/release/bootimage-utah-kernel.bin"
OUTPUT_PKG="$ROOT/utah_v1_signed.pkg"

if [[ ! -f "$KERNEL_BIN" ]]; then
  echo "[FATAL] Expected kernel binary missing: $KERNEL_BIN"
  exit 1
fi

if command -v openssl >/dev/null 2>&1; then
  echo "[UTAH-DEPLOY] Encrypting distribution artifact (AES-256-CBC)..."
  openssl enc -aes-256-cbc -salt -pbkdf2 \
    -in "$KERNEL_BIN" \
    -out "$OUTPUT_PKG"
  echo "[SUCCESS] Utah-Kernel manifest complete."
  echo "  Boot image: $KERNEL_BIN"
  echo "  Encrypted:  $OUTPUT_PKG"
else
  cp "$KERNEL_BIN" "$ROOT/utah_v1_release.bin"
  echo "[SUCCESS] Utah-Kernel manifest complete (openssl not found — copied unencrypted)."
  echo "  Boot image: $KERNEL_BIN"
  echo "  Copy:       $ROOT/utah_v1_release.bin"
fi

echo "  Emulate:    cargo run --release"
