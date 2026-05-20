# Genesis — Utah-OS Application Scaffold

Every Utah-OS app (calculator, browser, AI assistant) inherits from **`UtahApp`** — a universal intent interface so the OS can manifest, observe, and mutate app state—not only execute binaries.

## Layout

```
genesis/
├── src/
│   ├── core/
│   │   └── base_app.py      # UtahApp + VibeCodeApp example
│   └── apps/
│       ├── browser.py       # Utah-Browser scaffold
│       └── vibe_code.py     # Standalone vibe demo
└── README.md
```

## Run (development on host)

```bash
cd genesis
python -m src.apps.browser
python -m src.apps.vibe_code
```

## Ship to bare metal

1. Compile app logic to WASM (or wrap host bridge in future).
2. `python tools/utah-pack.py your_app.wasm`
3. Boot Utah-OS USB or EFI entry.

## Utah-Browser / Wry (roadmap)

Production Utah-Browser will embed **Wry** (lightweight WebView) to paint HTML into the Glass-Forge framebuffer without Chrome/Firefox overhead. The current `browser.py` is the intent scaffold and VRAM navigation hook.
