# Utah-OS Display Subsystem

Ring-0 **Unified Virtual Coordinate Topology**, **Sovereign EDID overrides**, and **intent-based app pinning** — no traditional window manager loop.

## Modules (`core/src/display/`)

| File | Role |
|------|------|
| `topology.rs` | Stitch monitors into one canvas; `resolve_physical_coordinates` |
| `edid.rs` | Parse/optimize 128-byte EDID; refresh ceilings (75 / 144 / 240 Hz tiers) |
| `window_manager.rs` | `ApplicationPinRule`, `StructuralWindowFrame`, border commit |
| `mod.rs` | Boot orchestration, global topology mutex, Glass-Forge composite |

## Boot sequence

After Glass-Forge splash (`utah_os::boot()`):

1. `initialize_unified_topology()` — two simulated heads (800×600 + 640×480, side-by-side)
2. `apply_edid_overrides()` — refresh ceilings from EDID profiles
3. `register_default_pinned_windows()` — browser → monitor 1, AI → highest Hz, HFS → monitor 0
4. `render_pinned_windows()` — accent borders on unified coordinates
5. `composite_primary_head_to_framebuffer()` — head 0 RGB → Glass-Forge BGRA (visible in QEMU)

Production GOP/DDC will replace simulated EDID and heap-backed VRAM with firmware framebuffers.

## Pin rules

| Rule | Behavior |
|------|----------|
| `StrictMonitorIndex(n)` | Origin at monitor `n` global offset |
| `HighestPerformanceDisplay` | Monitor with max `hardware_refresh_rate_hz` |
| `SpanAllAvailableMonitors` | Full `combined_virtual_width` × `combined_virtual_height` |

## Host API (WASM)

| Import | Signature | Returns |
|--------|-----------|---------|
| `get_canvas_dimensions` | `()` | `i64` — high 32 = width, low 32 = height |
| `pin_window_to_monitor` | `(monitor, width, height)` | — |
| `resolve_global_pixel` | `(global_x, global_y)` | `i64` packed head/local coords (0 = off-screen) |
| `refresh_display_pins` | `()` | — redraw borders + composite |

See [HOST_API.md](HOST_API.md).

## QEMU multi-monitor (experimental)

```bash
python tools/forge_iso.py   # Linux / WSL
qemu-system-x86_64 -cdrom target/utah-os.iso -m 1024 -display default,show-cursor=on -device secondary-vga
```

Utah-OS currently maps heads in software; secondary VGA is for future GOP binding.
