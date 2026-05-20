# Utah-Kernel Repository Architecture

```
utah-kernal/
├── core/                 # Ring-0 kernel (logic of the universe)
│   ├── src/
│   │   ├── main.rs       # Boot entry (mutated by utah-pack)
│   │   ├── ui.rs         # Bridge to Glass-Forge
│   │   ├── allocator.rs
│   │   ├── hfs.rs
│   │   ├── zero_point_net.rs
│   │   ├── chrono_scheduler.rs
│   │   ├── ghost_daemon.rs
│   │   └── ...
│   ├── Cargo.toml
│   └── .cargo/config.toml
├── ui/                   # Glass-Forge VRAM engine (visual manifestation)
│   └── src/
│       ├── lib.rs
│       ├── framebuffer.rs
│       └── glass.rs
├── tools/                # Compiler of reality
│   ├── utah-pack.py      # WASM → kernel embed → bootimage
│   ├── utah-deploy.sh    # Release forge + optional encryption
│   ├── utah_install.ps1  # Windows EFI infiltrator (dual-boot with Windows)
│   ├── create_utah_usb.ps1 # USB Ghost-Boot key (GRUB + kernel)
│   ├── grub/utah_grub.cfg
│   └── ghost-burner.md   # Ghost-Burner documentation
├── genesis/              # UtahApp scaffold (browser, vibe-code)
│   └── src/core/base_app.py
├── docs/
│   └── GHOST_BOOT.md     # Hypervisor + USB + Windows coexistence
├── manifest/             # Hardware resonance schemas
│   ├── m5-pebble.schema.json
│   └── m5-pebble.default.json
├── README.md             # Tier 1 technical codex
├── UTAH_OS.md
├── CIVILIAN_DOCUMENTATION.md
├── CHILD_MANUAL.md
└── MONETIZATION.md
```

## Build from workspace root

```bash
cd core
cargo build
cargo bootimage
cargo run
```

Or package an app:

```bash
python tools/utah-pack.py your_app.wasm
```

## Crates

| Crate | Path | Role |
|-------|------|------|
| `utah-kernel` | `core/` | Bare-metal OS + Wasmi + host calls |
| `glass-forge` | `ui/` | Direct-to-VRAM glass-morphic UI |
