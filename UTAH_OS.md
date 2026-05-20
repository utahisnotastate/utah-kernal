# Utah-OS (UTA H-OS): State-Space Reality Management Console

Utah-OS is the product layer on **[Utah-Kernel](https://github.com/utahisnotastate/utah-kernal)** — an intent-state environment, not a classic file-and-process manager.

**Version:** 1.0.0 · **Host API:** [docs/HOST_API.md](docs/HOST_API.md) · **Quick start:** [docs/QUICKSTART.md](docs/QUICKSTART.md)

## Competitive edge

| Feature | World-A (Linux / Windows) | Utah-OS |
| --- | --- | --- |
| **Execution** | Context-switching | WASM + direct host calls |
| **Persistence** | Hierarchical files | Holographic content-addressing |
| **Networking** | TCP/IP | Zero-Point resonance mesh |
| **Updates** | Full reinstall | Delta-wave HFS patches |
| **Introspection** | Reactive logs | Thermodynamic telemetry |
| **Security** | MMU page faults | WASM validation + host boundaries |
| **UI** | Compositor / WM | Glass-Forge direct-to-VRAM |

## Modules (source paths)

| Module | Path | Role |
|--------|------|------|
| Boot / config | `core/src/kernel_config.rs`, `core/src/utah_os.rs` | `UTAH_OS_MASTER_CONFIG`, boot orchestration |
| Allocator | `core/src/allocator.rs` | Heap |
| WASM | `core/src/wasm_runtime.rs` | Wasmi loader |
| Host calls | `core/src/system_calls.rs` | `utah_system` imports |
| HFS | `core/src/hfs.rs` | Content-addressed store |
| ZPN | `core/src/zero_point_net.rs` | Resonance mesh |
| Chrono | `core/src/chrono_scheduler.rs` | Predictive intents |
| Thermo | `core/src/thermodynamic_virtualizer.rs` | Idle telemetry |
| Delta | `core/src/delta_wave_patch.rs` | In-place patches |
| Ghost | `core/src/ghost_daemon.rs` | Suspend / freeze / `hlt` |
| UI bridge | `core/src/ui.rs` | Glass-Forge link |
| Glass-Forge | `ui/src/` | Framebuffer + glass + voxels |
| Genesis apps | `genesis/src/` | Python `UtahApp` scaffold |

## Ghost-Boot (Windows coexistence)

Does **not** format your Windows drive.

| Method | Tool |
|--------|------|
| USB key | `tools/create_utah_usb.ps1` |
| Internal EFI | `tools/utah_install.ps1` |

Architecture: [docs/GHOST_BOOT.md](docs/GHOST_BOOT.md)

## Guest host API (complete list)

See [docs/HOST_API.md](docs/HOST_API.md). Summary:

- **Display:** `print_text_to_screen`
- **HFS:** `save_hologram`, `load_hologram`
- **ZPN:** `broadcast`, `consume`, `tune_mesh`, `mesh_frequency`
- **Chrono:** `record_and_predict`, `take_staged_intent`
- **Thermo:** `read_thermodynamics`
- **Patch:** `apply_delta_patch`
- **Ghost:** `ghost_suspend`, `ghost_resume`, `register_wasm_snapshot`, `finalize_system_freeze`, `enter_phantom_sleep`
- **UI:** `render_interface_node`, `draw_voxel_cloud`

## Glass-Forge

- 800×600 BGRA buffer in `ui/src/framebuffer.rs`
- Boot splash: `ui/src/glass.rs` → `draw_boot_splash()`
- Dynamic clouds: `ui/src/voxel.rs` → `draw_dynamic_voxel_cloud()`
- No X11, Wayland, HTML, or Qt on bare metal

## Build

```bash
python tools/utah-pack.py app.wasm
./tools/utah-deploy.sh
cd core && cargo run --release
```

## Genesis (host development)

```bash
cd genesis
py -3 src/apps/browser.py
```

Apps use `UtahApp` in `genesis/src/core/base_app.py` — compile to WASM and pack for bare metal.

## v1.0.0 scope

**Included:** kernel, UI crate, tools, docs, Genesis scaffold, USB/EFI installers.

**Roadmap:** KVM + GPU passthrough for Windows games, NVMe HFS persistence, NIC driver, Wry browser → framebuffer.

[docs/RELEASE.md](docs/RELEASE.md)
