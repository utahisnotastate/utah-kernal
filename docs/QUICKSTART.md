# Utah-OS Quick Start

Get from zero to a bootable image in under 15 minutes (with toolchain installed).

## 1. Clone

```bash
git clone https://github.com/utahisnotastate/utah-kernal.git
cd utah-kernal
```

## 2. Install toolchain

**Windows (PowerShell):**

```powershell
# Rust nightly + bare-metal target
rustup toolchain install nightly
rustup component add rust-src llvm-tools-preview --toolchain nightly
rustup target add x86_64-unknown-none --toolchain nightly
cargo install bootimage

# QEMU: https://www.qemu.org/ then add to PATH
```

**Linux / macOS:** same `rustup` / `cargo install bootimage` commands; install `qemu-system-x86_64` via your package manager.

## 3. Build and emulate (no WASM app yet)

```bash
cd core
cargo bootimage
cargo run
```

QEMU opens with green VGA text and the Glass-Forge boot splash in kernel RAM.

## 4. Package a WebAssembly app

Your `.wasm` must:

- Export **`_start`** (function with no params)
- Export linear memory named **`memory`**
- Import host functions from module **`utah_system`** as needed (see [HOST_API.md](HOST_API.md))

```bash
cd ..
python tools/utah-pack.py path/to/your_app.wasm
# Windows: py -3 tools\utah-pack.py path\to\your_app.wasm

cd core
cargo run
```

## 5. Boot from USB (keeps Windows intact)

```powershell
# Administrator PowerShell
cd core
cargo bootimage --release
cd ..
.\tools\create_utah_usb.ps1
```

Reboot → boot menu → USB **UTAH-OS**. Internal Windows disk is **not** formatted.

## 6. Dual-boot on internal EFI (Windows stays)

```powershell
.\tools\utah_install.ps1
```

Adds **Utah-OS Reality Console** to BCD; reboot and pick from firmware menu.

## 7. Build bootable ISO (VirtualBox / VMware / QEMU)

**Linux / WSL** (requires `grub-mkrescue` or `xorriso`):

```bash
python tools/forge_iso.py
qemu-system-x86_64 -cdrom target/utah-os.iso -m 512 -vga std
```

**Dual-head topology test** (software-unified canvas; secondary VGA for future GOP):

```bash
qemu-system-x86_64 -cdrom target/utah-os.iso -m 1024 -display default,show-cursor=on -device secondary-vga
```

Pinned window borders composite onto the primary Glass-Forge buffer — see [DISPLAY.md](DISPLAY.md).

## 8. Try Genesis apps (host dev)

```bash
cd genesis
py -3 src/apps/browser.py
py -3 src/apps/vibe_code.py "open calculator"
```

## Next reading

- [HOST_API.md](HOST_API.md) — all system calls
- [DISPLAY.md](DISPLAY.md) — multi-monitor topology and pinning
- [GHOST_BOOT.md](GHOST_BOOT.md) — hypervisor roadmap
- [../REPO_ARCHITECTURE.md](../REPO_ARCHITECTURE.md) — full tree
