# Ghost-Burner — Zero-Click Utah-OS Installer

World-A installers use MSI wizards and `apt-get`. **Ghost-Burner** injects Utah-OS into the **EFI boot chain** beside Windows — your games and apps stay on existing NTFS volumes.

## Windows: `utah_install.ps1`

Run **PowerShell as Administrator** from the repo root:

```powershell
cd core
cargo bootimage --release
cd ..
.\tools\utah_install.ps1
```

### What it does

1. Locates `core/target/.../bootimage-utah-kernel.bin` (builds if missing — you must build first).
2. Uses the existing **EFI System Partition** when possible, or creates a **512MB FAT32** `UTAH-OS` partition.
3. Copies the kernel to `\EFI\UtahOS\boot\BOOTX64.EFI`.
4. Registers a **BCD** entry: **Utah-OS Reality Console**.
5. Reboot → pick Utah-OS from the firmware/boot menu.

### Safety

- Does **not** delete Windows partitions.
- Requires explicit admin consent (UAC).
- Back up important data before modifying boot configuration.

## Linux / RAM installer (future)

The original PXE/RAM mirror flow remains the long-term path for bit-level dual-boot without Windows BCD.

## Kernel integration

| Host import | Role |
|-------------|------|
| `register_wasm_snapshot` | Queue WASM linear memory before freeze |
| `finalize_system_freeze` | HFS commit + `cli`/`hlt` (never returns) |
| `ghost_suspend` / `ghost_resume` | Per-session state collapse |

## Related

- [REPO_ARCHITECTURE.md](../REPO_ARCHITECTURE.md)
- [manifest/m5-pebble.default.json](../manifest/m5-pebble.default.json)
