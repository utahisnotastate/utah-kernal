# Utah-Kernel Repository Architecture

**Release:** v1.0.0 · **Repo:** [github.com/utahisnotastate/utah-kernal](https://github.com/utahisnotastate/utah-kernal)

## Directory tree

```
utah-kernal/
├── core/                      # Ring-0 kernel (crate: utah-kernel)
│   ├── src/
│   │   ├── main.rs            # Entry (rewritten by utah-pack)
│   │   ├── boot.asm           # Multiboot2 header
│   │   ├── system_calls.rs    # utah_system host API
│   │   ├── wasm_runtime.rs
│   │   ├── hfs.rs
│   │   ├── zero_point_net.rs
│   │   ├── chrono_scheduler.rs
│   │   ├── ghost_daemon.rs
│   │   ├── thermodynamic_virtualizer.rs
│   │   ├── delta_wave_patch.rs
│   │   ├── kernel_config.rs
│   │   ├── utah_os.rs
│   │   └── ui.rs              # → glass-forge crate
│   ├── Cargo.toml
│   └── .cargo/config.toml
├── ui/                        # Glass-Forge (crate: glass-forge)
│   └── src/
│       ├── lib.rs
│       ├── framebuffer.rs
│       ├── glass.rs
│       └── voxel.rs
├── tools/
│   ├── utah-pack.py
│   ├── utah-deploy.sh
│   ├── create_utah_usb.ps1
│   ├── utah_install.ps1
│   ├── grub/utah_grub.cfg
│   └── ghost-burner.md
├── genesis/                   # Host-side UtahApp scaffold
│   └── src/
│       ├── core/base_app.py
│       └── apps/{browser,vibe_code}.py
├── manifest/
│   ├── m5-pebble.schema.json
│   └── m5-pebble.default.json
├── docs/
│   ├── QUICKSTART.md
│   ├── HOST_API.md
│   ├── RELEASE.md
│   └── GHOST_BOOT.md
├── README.md
├── CHANGELOG.md
├── LICENSE
├── CONTRIBUTING.md
├── UTAH_OS.md
├── CIVILIAN_DOCUMENTATION.md
├── CHILD_MANUAL.md
├── MONETIZATION.md
├── Cargo.toml                 # Workspace root
└── rust-toolchain.toml
```

## Workspace crates

| Crate | Path | Role |
|-------|------|------|
| `utah-kernel` | `core/` | Bare-metal OS + Wasmi + host calls |
| `glass-forge` | `ui/` | Direct-to-VRAM UI |

```bash
cargo check -p utah-kernel -p glass-forge
```

## Data flow

```
WASM guest
  → import utah_system::*
    → core/src/system_calls.rs
      → hfs | zpn | chrono | ghost | ui (glass-forge)
```

## Boot flow

1. `_start` in `core/src/main.rs`
2. `allocator::initialize_system_heap()`
3. `utah_os::boot()` — config, Glass-Forge splash, subsystems
4. Optional embedded WASM via `wasm_runtime::run_web_assembly_program`
5. Idle: `utah_os::service_idle()`

## Packaging flow

```
app.wasm → tools/utah-pack.py → core/src/main.rs → cargo bootimage → .bin
```

## Install flow (Windows)

```
.bin → create_utah_usb.ps1  → USB EFI boot
.bin → utah_install.ps1     → internal dual-boot
```
