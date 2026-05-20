# Changelog

All notable changes to [utah-kernal](https://github.com/utahisnotastate/utah-kernal) are documented here.

## [Unreleased]

### Added

- Documentation sync across README, UTAH_OS, RELEASE, HOST_API, DISPLAY, architecture docs

- **Unified display topology** (`core/src/display/`): multi-head virtual canvas, EDID refresh optimization, intent-based window pinning
- Host calls 21–24: `get_canvas_dimensions`, `pin_window_to_monitor`, `resolve_global_pixel`, `refresh_display_pins`
- [docs/DISPLAY.md](docs/DISPLAY.md) — architecture and QEMU notes
- Theme Registry Matrix (`ui/src/theme.rs`): Dark, Golden, Light, Linda, Occult presets
- Runtime vibe-code color parser (`apply_vibe_theme` host call)
- Utah-OS desktop manifold renderer (taskbar, panels, browser workspace)
- `tools/forge_iso.py` — GRUB2 staging + bootable ISO for VMs

## [1.0.0] - 2026-05-20

First public release: **Utah-Kernel** bare-metal core + **Utah-OS** product layer.

### Added

- **Ring-0 kernel** (`core/`): heap, Wasmi WASM runtime, Multiboot2 header, VGA text
- **Holographic File System (HFS)**: content-addressed, deduplicated storage
- **Zero-Point Network**: resonance-frequency intent messaging
- **Chrono-Scheduler**: predictive intent pre-staging
- **Thermodynamic Virtualizer**: idle telemetry
- **Delta-Wave patching**: in-place XOR delta commits to HFS
- **Ghost-Daemon**: suspend/resume, phantom sleep, `finalize_system_freeze`
- **Glass-Forge UI** (`ui/`): direct-to-VRAM glass panels and voxel clouds
- **Host API**: 18 `utah_system` imports (see [docs/HOST_API.md](docs/HOST_API.md))
- **Tools**: `utah-pack.py`, `utah-deploy.sh`, `utah_install.ps1`, `create_utah_usb.ps1`
- **Genesis**: Python `UtahApp` scaffold, Utah-Browser, VibeCode demo
- **Docs**: tiered README, civilian, child, Ghost-Boot, monetization, architecture

### Known limitations (v1.0.0)

- No physical NIC, NVMe, or KVM/Windows capsule yet (documented roadmap)
- `bootimage` output may require platform-specific EFI wrapping on some PCs
- WASM guest must export `_start` and `memory`; sample 8-byte header is not runnable alone
- Glass-Forge uses in-RAM framebuffer until bootloader maps real VRAM

[1.0.0]: https://github.com/utahisnotastate/utah-kernal/releases/tag/v1.0.0
