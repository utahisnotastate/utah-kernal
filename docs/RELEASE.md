# Release v1.0.0 — Utah-Kernel / Utah-OS

**Repository:** [github.com/utahisnotastate/utah-kernal](https://github.com/utahisnotastate/utah-kernal)

This is the first **public release** of the project: a bare-metal WebAssembly unikernel that grew into **Utah-OS** (kernel + UI + tooling + app scaffold).

## What “complete” means for v1.0.0

| Area | Shipped | Not yet (roadmap) |
|------|---------|-------------------|
| Boot | Multiboot2, bootimage, QEMU | Signed PE/EFI on all OEM firmware |
| Runtime | Wasmi, 18 host imports | WASI full profile |
| Storage | HFS in RAM | NVMe / disk persistence |
| Network | ZPN loopback | Real NIC DMA |
| UI | Glass-Forge 800×600 buffer | GPU MMIO from bootloader |
| Windows coexistence | USB + EFI installers | KVM + GPU passthrough capsule |
| Apps | Genesis Python scaffold | Wry browser in framebuffer |

The release is **complete as an open-source foundation**: build, document, boot, extend. It is **not** a replacement for Windows or Linux for daily desktop use.

## Post-1.0.0 on `main` (not yet tagged)

| Area | Added on `main` |
|------|-----------------|
| Host API | Calls 19–24: themes + unified display (`get_canvas_dimensions`, `pin_window_to_monitor`, `resolve_global_pixel`, `refresh_display_pins`) |
| UI | Theme presets (Dark/Golden/Light/Linda/Occult), vibe parser, desktop manifold |
| Display | `core/src/display/` — virtual topology, EDID optimization, intent-based pinning |
| Tools | `forge_iso.py` for VM ISO images |
| Docs | [DISPLAY.md](DISPLAY.md), [THEMES.md](THEMES.md) |

Clone **`main`** for the latest behavior; pin **`v1.0.0`** for the first tagged baseline. See [CHANGELOG.md](../CHANGELOG.md#unreleased).

## Artifacts

| Artifact | Path |
|----------|------|
| Kernel binary | `core/target/.../bootimage-utah-kernel.bin` |
| Encrypted package (optional) | `utah_v1_signed.pkg` via `tools/utah-deploy.sh` |
| USB layout | `tools/create_utah_usb.ps1` |

## Verify build

```bash
cargo check -p utah-kernel -p glass-forge
cd core && cargo bootimage && cargo run
```

## Upgrade path

See [CHANGELOG.md](../CHANGELOG.md). Future tags will add hypervisor, block storage, and automated WASM discovery on disk.

## Support

Open issues on GitHub for bugs and feature requests. Read [QUICKSTART.md](QUICKSTART.md) before filing “won’t boot” reports (include QEMU vs USB vs EFI path).
