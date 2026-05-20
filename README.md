# Utah-Kernel / Utah-OS

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-nightly-orange)](rust-toolchain.toml)
[![Release](https://img.shields.io/badge/release-v1.0.0-green)](CHANGELOG.md)

**Repository:** [github.com/utahisnotastate/utah-kernal](https://github.com/utahisnotastate/utah-kernal)

> I set out to build a state-of-the-art OS. The only thing I knew for sure was that I needed a **kernel** first. I got impatient and turned that kernel into an OS as well — so this repo is **Utah-Kernel** and **Utah-OS**.

A **bare-metal**, **Ring-0** unikernel that runs **WebAssembly** with explicit host functions instead of a traditional user/kernel split. Includes **Glass-Forge** (direct-to-VRAM UI), **Ghost-Boot** installers for USB/Windows coexistence, and a **Genesis** app scaffold.

**New here?** → [docs/QUICKSTART.md](docs/QUICKSTART.md)

---

## What ships in v1.0.0

| Component | Description |
|-----------|-------------|
| **Kernel** (`core/`) | Heap, Wasmi runtime, Multiboot2, 18 `utah_system` host calls |
| **HFS** | Content-addressed storage (DJB2 resonance signatures) |
| **Zero-Point Network** | Resonance-frequency messaging (no TCP/IP in kernel) |
| **Chrono-Scheduler** | Predictive intent pre-staging |
| **Ghost-Daemon** | State collapse, phantom sleep, system freeze |
| **Glass-Forge** (`ui/`) | Themed desktop manifold (Dark/Golden/Light/Linda/Occult) + vibe parser |
| **Tools** | `utah-pack`, `utah-deploy`, USB + EFI Ghost-Burner scripts |
| **Genesis** | Python `UtahApp` / Utah-Browser / VibeCode demos |

**Roadmap (not v1.0.0):** KVM Windows capsule, GPU passthrough, NVMe, physical NIC, Wry browser blit.

Details: [docs/RELEASE.md](docs/RELEASE.md) · [CHANGELOG.md](CHANGELOG.md)

---

## Quick start

```bash
git clone https://github.com/utahisnotastate/utah-kernal.git
cd utah-kernal

# Toolchain (once)
rustup toolchain install nightly
rustup component add rust-src llvm-tools-preview --toolchain nightly
rustup target add x86_64-unknown-none --toolchain nightly
cargo install bootimage

# Build & run in QEMU
cd core
cargo bootimage
cargo run
```

**Package your WASM app:**

```bash
python tools/utah-pack.py path/to/app.wasm   # Windows: py -3 tools\utah-pack.py ...
cd core && cargo run
```

**Boot from USB (does not format internal Windows drive):**

```powershell
# Admin PowerShell, after: cd core; cargo bootimage --release
.\tools\create_utah_usb.ps1
```

Full guide: [docs/QUICKSTART.md](docs/QUICKSTART.md)

---

## Architecture

```
UEFI / QEMU
    └── Utah-Kernel (Ring-0)
            ├── Wasmi (WASM guests)
            ├── HFS · ZPN · Chrono · Ghost-Daemon
            └── Glass-Forge → linear framebuffer
```

- **No** Linux / Windows / X11 / Wayland required on bare metal.
- Guests call **`utah_system::*`** host functions — see [docs/HOST_API.md](docs/HOST_API.md).
- Windows coexistence: [docs/GHOST_BOOT.md](docs/GHOST_BOOT.md).

```
utah-kernal/
├── core/       # utah-kernel crate (Ring-0)
├── ui/         # glass-forge crate (VRAM UI)
├── tools/      # pack, deploy, USB/EFI installers
├── genesis/    # UtahApp Python scaffold
├── manifest/   # M5-Pebble hardware JSON
└── docs/       # QUICKSTART, HOST_API, RELEASE, GHOST_BOOT
```

[REPO_ARCHITECTURE.md](REPO_ARCHITECTURE.md)

---

## Prerequisites

| Tool | Purpose |
|------|---------|
| [rustup](https://rustup.rs/) nightly | Bare-metal Rust (`rust-toolchain.toml`) |
| [bootimage](https://github.com/rust-osdev/bootimage) | Bootable disk image |
| [QEMU](https://www.qemu.org/) | Emulation (`qemu-system-x86_64`) |
| Python 3 | `utah-pack.py`, Genesis demos |
| PowerShell (Admin) | Windows USB / EFI installers |

---

## Host API (summary)

Module: **`utah_system`**. Guest must export **`memory`** and **`_start`**.

| Category | Imports |
|----------|---------|
| Display | `print_text_to_screen` |
| Storage | `save_hologram`, `load_hologram` |
| Network | `broadcast`, `consume`, `tune_mesh`, `mesh_frequency` |
| Scheduler | `record_and_predict`, `take_staged_intent` |
| Energy | `read_thermodynamics` |
| Updates | `apply_delta_patch` |
| Ghost | `ghost_suspend`, `ghost_resume`, `register_wasm_snapshot`, `finalize_system_freeze`, `enter_phantom_sleep` |
| UI | `render_interface_node`, `draw_voxel_cloud`, `set_theme_preset`, `apply_vibe_theme` |

Complete tables: [docs/HOST_API.md](docs/HOST_API.md)

---

## Tools

| Script | Use |
|--------|-----|
| [tools/utah-pack.py](tools/utah-pack.py) | Embed `.wasm` → `core/src/main.rs` → `cargo bootimage` |
| [tools/forge_iso.py](tools/forge_iso.py) | Build `target/utah-os.iso` for VirtualBox/VMware/QEMU |
| [tools/utah-deploy.sh](tools/utah-deploy.sh) | Release build + optional AES package |
| [tools/create_utah_usb.ps1](tools/create_utah_usb.ps1) | USB Ghost-Boot key (FAT32 + GRUB + kernel) |
| [tools/utah_install.ps1](tools/utah_install.ps1) | Internal EFI dual-boot beside Windows |

[tools/README.md](tools/README.md) · [tools/ghost-burner.md](tools/ghost-burner.md)

---

## Documentation

| Audience | Document |
|----------|----------|
| Developers | [README.md](README.md) (this file), [docs/HOST_API.md](docs/HOST_API.md), [UTAH_OS.md](UTAH_OS.md) |
| Quick setup | [docs/QUICKSTART.md](docs/QUICKSTART.md) |
| Release scope | [docs/RELEASE.md](docs/RELEASE.md), [CHANGELOG.md](CHANGELOG.md) |
| Non-technical | [CIVILIAN_DOCUMENTATION.md](CIVILIAN_DOCUMENTATION.md) |
| Young learners | [CHILD_MANUAL.md](CHILD_MANUAL.md) |
| Business | [MONETIZATION.md](MONETIZATION.md) |
| Contributing | [CONTRIBUTING.md](CONTRIBUTING.md) |

---

## Build outputs

| Profile | Typical path |
|---------|----------------|
| Debug | `core/target/x86_64-unknown-none/debug/bootimage-utah-kernel.bin` |
| Release | `core/target/x86_64-unknown-none/release/bootimage-utah-kernel.bin` |

```bash
cd core
cargo bootimage --release
```

---

## Status

**v1.0.0** is a **complete open-source foundation**: documented, buildable, bootable in QEMU, packagable via WASM, installable via USB/EFI scripts.

It is a **high-density research prototype**, not a daily-driver desktop OS. Use QEMU and USB paths for safe testing.

---

## License

[MIT License](LICENSE) — Copyright (c) 2026 Utah-Kernel / Utah-OS Contributors
