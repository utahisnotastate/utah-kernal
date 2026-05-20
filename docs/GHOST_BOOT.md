# Ghost-Boot Architecture

Utah-OS does **not** reformat your Windows drive. It coexists via **UEFI handoff** and (roadmap) a **bare-metal first-stage** that can host Windows inside a hardware-accelerated container.

## Deployment modes

| Mode | Tool | Use case |
|------|------|----------|
| **USB Ghost-Key** | `tools/create_utah_usb.ps1` | Dad-proof: boot Utah from USB, zero internal disk changes |
| **EFI infiltrator** | `tools/utah_install.ps1` | Dual-boot entry beside Windows on internal ESP |
| **WASM forge** | `tools/utah-pack.py` | Embed apps into kernel image |

## Boot order (target architecture)

```
UEFI firmware
    └── Utah-OS Aegis-Kernel (Ring-0, ~5ms init target)
            ├── Glass-Forge → VRAM / framebuffer
            ├── HFS + Zero-Point mesh + Chrono-Scheduler
            └── Windows capsule (roadmap: KVM + GPU passthrough)
```

### Windows compatibility (roadmap)

- **KVM-based GPU passthrough**: Windows runs as a guest with near-native game performance.
- **Utah-OS as Type-1 layer**: Kernel boots first; detects Windows partition; encapsulates NTFS boot chain in a VM with VFIO GPU assignment.
- **Control plane**: Utah monitors guest physical memory ranges for optimization, invisible assistants, and debug hooks Windows cannot see from inside the guest.

**Current repo status:** Ring-0 kernel, host calls, USB/EFI installers, and Genesis app scaffold ship today. Full hypervisor + VFIO is documented here for integration phases.

## USB layout (after `create_utah_usb.ps1`)

```
USB (FAT32 UTAH-OS)
├── EFI/BOOT/BOOTX64.EFI      # Utah-Kernel
├── EFI/UtahOS/boot/BOOTX64.EFI
├── boot/grub/grub.cfg
├── UTAH/utah-kernel.bin
└── README.txt
```

## Genesis apps (host-side)

Python apps under `genesis/` use `UtahApp` intent interface. They compile to WASM via your toolchain and pack with `utah-pack.py`, or run on a connected host during development.

## Safety

- USB creator requires typing **YES** before format.
- Always back up data before changing EFI/BCD.
- Game partitions on NTFS are never targeted by the USB formatter.
