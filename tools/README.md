# Utah-OS Tools

| Tool | Purpose |
|------|---------|
| [`utah-pack.py`](utah-pack.py) | Embed `.wasm` into `core/src/main.rs` and run `cargo bootimage` |
| [`utah-deploy.sh`](utah-deploy.sh) | Release forge + optional AES packaging |
| [`utah_install.ps1`](utah_install.ps1) | Windows UEFI Ghost-Burner (Admin PowerShell) |
| [`create_utah_usb.ps1`](create_utah_usb.ps1) | USB Ghost-Key installer (GRUB + kernel, dad-proof) |
| [`forge_iso.py`](forge_iso.py) | GRUB2 + ISO image for VM testing (see [docs/DISPLAY.md](../docs/DISPLAY.md) for dual-head QEMU) |
| [`ghost-burner.md`](ghost-burner.md) | Ghost-Burner dual-boot documentation |

```bash
python tools/utah-pack.py ../path/to/app.wasm
./tools/utah-deploy.sh
```
