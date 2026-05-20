#!/usr/bin/env python3
"""
Utah-OS Monolithic ISO Forge Utility
------------------------------------
Packages the compiled bare-metal kernel, builds a GRUB2 boot layout, and
produces a bootable .iso for VirtualBox, VMware, and QEMU.
"""

from __future__ import annotations

import os
import shutil
import subprocess
import sys
from pathlib import Path


def find_repo_root() -> Path:
    here = Path(__file__).resolve().parent
    root = here.parent
    if (root / "core" / "Cargo.toml").is_file():
        return root
    raise SystemExit("[FATAL] Run from utah-kernal repository (tools/forge_iso.py).")


def run_step(command: list[str], label: str, cwd: Path) -> None:
    print(f"[FORGE] {label}...")
    try:
        subprocess.run(command, cwd=cwd, check=True)
    except FileNotFoundError:
        print(f"[FATAL] Command not found: {command[0]}")
        sys.exit(1)
    except subprocess.CalledProcessError as exc:
        print(f"[FATAL] Failed: {label}")
        if exc.stderr:
            print(exc.stderr.decode("utf-8", errors="replace"))
        sys.exit(1)


def find_kernel_binary(core_dir: Path) -> Path:
    candidates = [
        core_dir / "target" / "x86_64-unknown-none" / "release" / "utah-kernel",
        core_dir / "target" / "x86_64-unknown-none" / "debug" / "utah-kernel",
        core_dir / "target" / "x86_64-unknown-none" / "release" / "bootimage-utah-kernel.bin",
        core_dir / "target" / "x86_64-unknown-none" / "debug" / "bootimage-utah-kernel.bin",
    ]
    for path in candidates:
        if path.is_file():
            return path
    raise FileNotFoundError("Build kernel first: cd core && cargo build --release")


def main() -> None:
    print("=" * 50)
    print("Utah-OS ISO compilation pipeline")
    print("=" * 50)

    root = find_repo_root()
    core = root / "core"
    staging = root / "iso_staging_workspace"
    grub_dir = staging / "boot" / "grub"
    out_dir = root / "target"
    out_dir.mkdir(parents=True, exist_ok=True)
    output_iso = out_dir / "utah-os.iso"

    run_step(
        ["cargo", "build", "--release"],
        "Rust bare-metal release build",
        core,
    )

    kernel_src = find_kernel_binary(core)
    if staging.exists():
        shutil.rmtree(staging)
    grub_dir.mkdir(parents=True, exist_ok=True)
    (staging / "boot").mkdir(parents=True, exist_ok=True)

    kernel_dst = staging / "boot" / "utah-kernel.bin"
    shutil.copy2(kernel_src, kernel_dst)
    print(f"[FORGE] Kernel copied: {kernel_dst}")

    grub_cfg = staging / "boot" / "grub" / "grub.cfg"
    template = root / "tools" / "grub" / "utah_grub.cfg"
    if template.is_file():
        shutil.copy2(template, grub_cfg)
    else:
        grub_cfg.write_text(
            """set timeout=3
set default=0
menuentry "Utah-OS Sovereign Reality Environment" {
    multiboot2 /boot/utah-kernel.bin
    boot
}
""",
            encoding="utf-8",
        )
    print(f"[FORGE] GRUB config: {grub_cfg}")

    # UEFI fallback path
    efi_boot = staging / "EFI" / "BOOT"
    efi_boot.mkdir(parents=True, exist_ok=True)
    shutil.copy2(kernel_dst, efi_boot / "BOOTX64.EFI")

    iso_built = False
    for cmd, label in [
        (["grub-mkrescue", "-o", str(output_iso), str(staging)], "grub-mkrescue"),
        (
            ["xorriso", "-as", "mkisofs", "-r", "-J", "-o", str(output_iso), str(staging)],
            "xorriso",
        ),
    ]:
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            print(f"[FORGE] ISO created via {label}")
            iso_built = True
            break
        except (FileNotFoundError, subprocess.CalledProcessError):
            continue

    if not iso_built:
        print("[WARN] grub-mkrescue/xorriso not available.")
        print("       Staging tree left at:", staging)
        print("       On Linux: sudo apt install grub-pc-bin xorriso && re-run.")
        print("       On Windows: use WSL or bootimage USB path (create_utah_usb.ps1).")
        sys.exit(1)

    shutil.rmtree(staging, ignore_errors=True)

    print()
    print("=" * 50)
    print("SUCCESS: bootable ISO ready")
    print(f"  {output_iso}")
    print("=" * 50)
    print("QEMU:")
    print(f"  qemu-system-x86_64 -cdrom {output_iso} -m 512 -vga std")


if __name__ == "__main__":
    main()
