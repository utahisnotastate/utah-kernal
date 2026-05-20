# Utah-OS Tools

| Tool | Purpose |
|------|---------|
| [`utah-pack.py`](utah-pack.py) | Embed `.wasm` into `core/src/main.rs` and run `cargo bootimage` |
| [`utah-deploy.sh`](utah-deploy.sh) | Release forge + optional AES packaging |
| [`utah_install.ps1`](utah_install.ps1) | Windows UEFI Ghost-Burner (Admin PowerShell) |
| [`ghost-burner.md`](ghost-burner.md) | Ghost-Burner dual-boot documentation |

```bash
python tools/utah-pack.py ../path/to/app.wasm
./tools/utah-deploy.sh
```
