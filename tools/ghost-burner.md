# Ghost-Burner — Zero-Click Utah-OS Installer

World-A installers use `apt-get`, MSI wizards, or drag-and-drop `.dmg` files. **Ghost-Burner** is a self-bootstrapping pre-boot environment that mirrors the Utah-Kernel into an existing machine without replacing the user's primary OS.

## Concept

1. User runs `Utah-OS-Installer` (future host binary built from this spec).
2. The machine reboots into a **temporary Utah partition in RAM** (PXE or USB pre-boot).
3. Ghost-Burner performs a **bit-level mirror** of the target drive and injects:
   - Utah-Kernel boot sector entry (sovereign dual-boot)
   - HFS resonance table stub
   - Glass-Forge framebuffer config from `manifest/m5-pebble.json`
4. User reboots; GRUB/multiboot menu offers **Utah-OS** alongside Windows/Linux.

## Status

This repository ships the **kernel and forge tools** (`utah-pack`, `utah-deploy`). The Ghost-Burner host executable (Windows `.exe` / Linux ELF) is the next product artifact — it will wrap:

- `tools/utah-pack.py` — embed WASM payloads
- `tools/utah-deploy.sh` — forge signed `utah_v1_signed.pkg`
- `core/target/.../bootimage-utah-kernel.bin` — bootable kernel

## Safety

Ghost-Burner must never overwrite user data without explicit consent. Production builds require:

- Drive snapshot backup
- Checksum verification of injected boot sectors
- Signed Utah-Kernel packages only

## Related

- [REPO_ARCHITECTURE.md](../REPO_ARCHITECTURE.md)
- [manifest/m5-pebble.schema.json](../manifest/m5-pebble.schema.json)
