# Utah-OS Host API (`utah_system`)

WebAssembly guests import functions from module **`utah_system`**. All pointer/length pairs refer to **guest linear memory** export **`memory`**.

Register new capabilities in `core/src/system_calls.rs`.

**Current `main` branch:** **24** host imports (calls 1–24). Tagged **v1.0.0** shipped 18 imports; calls 19–24 and display topology are post-release — see [CHANGELOG.md](../CHANGELOG.md#unreleased).

## Requirements

| Export | Type | Notes |
|--------|------|-------|
| `memory` | memory | Linear memory for buffers |
| `_start` | `func () -> ()` | Entry point after instantiation |

## System calls

### Display

| Import | Signature | Returns | Description |
|--------|-----------|---------|-------------|
| `print_text_to_screen` | `(i32 ptr, i32 len)` | — | UTF-8 text to VGA (0xB8000) |
| `get_canvas_dimensions` | `()` | `i64` | Unified canvas width (high 32) and height (low 32) |
| `pin_window_to_monitor` | `(i32 monitor, i32 w, i32 h)` | — | Pin guest window frame to monitor index |
| `resolve_global_pixel` | `(i32 gx, i32 gy)` | `i64` | Map global coords to head/local pixel (0 = off-screen) |
| `refresh_display_pins` | `()` | — | Redraw pinned borders and composite to Glass-Forge |

See [DISPLAY.md](DISPLAY.md).

### Holographic File System

| Import | Signature | Returns | Description |
|--------|-----------|---------|-------------|
| `save_hologram` | `(i32 ptr, i32 len)` | `i64` | Store bytes; returns resonance signature (0 on error) |
| `load_hologram` | `(i64 sig, i32 dest)` | `i32` | Bytes written into guest memory |

### Zero-Point Network

| Import | Signature | Returns | Description |
|--------|-----------|---------|-------------|
| `broadcast` | `(i64 freq, i32 ptr, i32 len)` | — | Headerless intent broadcast |
| `consume` | `(i32 dest)` | `i32` | Next intent for local frequency → guest RAM |
| `tune_mesh` | `(i64 freq)` | — | Retune local resonance |
| `mesh_frequency` | `()` | `i64` | Current local frequency |

Default mesh frequency after boot: **7830000000** (from `kernel_config.rs`). Loopback when broadcast target matches local tune.

### Chrono-Scheduler

| Import | Signature | Returns | Description |
|--------|-----------|---------|-------------|
| `record_and_predict` | `(i32 action_id)` | `i64` | Record action; return predicted next id (0 = none) |
| `take_staged_intent` | `()` | `i64` | Pre-staged intent id (0 = none) |

Example transitions: `1→2→3→4`, `10→11→12`.

### Thermodynamic Virtualizer

| Import | Signature | Returns | Description |
|--------|-----------|---------|-------------|
| `read_thermodynamics` | `()` | `i64` | Packed telemetry (high 32 = idle ticks, low 32 = noise index) |

### Delta-Wave patching

| Import | Signature | Returns | Description |
|--------|-----------|---------|-------------|
| `apply_delta_patch` | `(i32 base_ptr, i32 base_len, i32 delta_ptr, i32 delta_len)` | `i64` | XOR-delta patch committed to HFS; returns signature |

### Ghost-Daemon

| Import | Signature | Returns | Description |
|--------|-----------|---------|-------------|
| `ghost_suspend` | `(i32 ptr, i32 len)` | `i64` | Collapse state to HFS |
| `ghost_resume` | `(i32 dest)` | `i32` | Restore last ghost state |
| `register_wasm_snapshot` | `(i32 ptr, i32 len)` | — | Queue segment before freeze |
| `finalize_system_freeze` | `()` | `i64` | Snapshot all → HFS → **cpu halt** (never returns) |
| `enter_phantom_sleep` | `()` | `i64` | Void sleep (never returns) |

### Glass-Forge UI

| Import | Signature | Returns | Description |
|--------|-----------|---------|-------------|
| `render_interface_node` | `(i32 x, i32 y, i32 intensity)` | — | Glass voxel (0–255 intensity) |
| `draw_voxel_cloud` | `(i32 ox, i32 oy, i32 vx, i32 vy, i32 intensity)` | — | Particle cloud along vector |
| `set_theme_preset` | `(preset_id: i32)` | — | 0–4: dark, golden, light, linda, occult |
| `apply_vibe_theme` | `(ptr, len)` | — | Parse vibe string; redraw desktop |

See [THEMES.md](THEMES.md).

## WAT import examples

```wat
(import "utah_system" "print_text_to_screen" (func $print (param i32 i32)))
(import "utah_system" "save_hologram" (func $save (param i32 i32) (result i64)))
(import "utah_system" "record_and_predict" (func $predict (param i32) (result i64)))
```

## Limits (current)

| Operation | Max bytes per call |
|-----------|-------------------|
| print | 4096 |
| hologram save / broadcast / delta | 65536 |
| ghost suspend / snapshot | 262144 |
