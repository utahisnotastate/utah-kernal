# Utah-OS Tools

| Tool | Purpose |
|------|---------|
| [`utah-pack.py`](utah-pack.py) | Embed `.wasm` into `core/src/main.rs` and run `cargo bootimage` |
| [`utah-deploy.sh`](utah-deploy.sh) | Release forge + optional AES packaging |
| [`ghost-burner.md`](ghost-burner.md) | Ghost-Burner zero-click installer specification |

```bash
python tools/utah-pack.py ../path/to/app.wasm
./tools/utah-deploy.sh
```
