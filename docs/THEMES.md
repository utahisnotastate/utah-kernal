# Utah-OS Theme Registry

Glass-Forge uses **`SystemTheme`** presets defined in `ui/src/theme.rs`.

## Presets

| ID | Name | Character |
|----|------|-----------|
| 0 | Dark | Slate + cyan (default) |
| 1 | Golden | Bronze + gold accents |
| 2 | Light | Daylight canvas |
| 3 | Linda | Synthwave violet / rose |
| 4 | Occult | Void crimson |

## Vibe-code runtime overrides

Semicolon-separated assignments:

```text
primary_accent: 250,204,21; secondary_accent: 236,72,153;
```

Keys: `background`, `surface`, `primary_accent`, `secondary_accent`, `interactive_element`, `text_primary`.

### From WASM

```wat
(import "utah_system" "set_theme_preset" (func (param i32)))
(import "utah_system" "apply_vibe_theme" (func (param i32 i32)))
```

### From kernel

```rust
crate::theme::set_preset(crate::theme::ThemePreset::Golden);
crate::theme::apply_vibe_modification("primary_accent: 220,38,38;");
crate::ui::render_boot_splash();
```

## Desktop manifold

`render_total_os_manifold` draws:

- Full-screen background
- Top taskbar + accent strip
- Calculator panel, HFS explorer, browser workspace
- Focus highlight on primary panel

Triggered automatically at boot via `utah_os::boot()`.
