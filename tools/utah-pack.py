#!/usr/bin/env python3
"""
Utah-Kernel Packaging Utility (utah-pack)
-----------------------------------------
Ingests a .wasm payload, injects it into core/src/main.rs, and runs cargo bootimage.
"""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

BYTES_PER_LINE = 16


def find_repo_root() -> Path:
    here = Path(__file__).resolve().parent
    root = here.parent
    if (root / "core" / "Cargo.toml").is_file():
        return root
    raise SystemExit("[FATAL ERROR] Run from the utah-kernal repository (tools/utah-pack.py).")


def find_core_dir() -> Path:
    return find_repo_root() / "core"


def format_rust_byte_array(wasm_bytes: bytes) -> str:
    if not wasm_bytes:
        return ""
    hex_items = [f"0x{b:02x}" for b in wasm_bytes]
    lines: list[str] = []
    for index in range(0, len(hex_items), BYTES_PER_LINE):
        chunk = ", ".join(hex_items[index : index + BYTES_PER_LINE])
        lines.append(f"        {chunk},")
    return "\n".join(lines)


def build_kernel_source(byte_array_body: str) -> str:
    return f"""#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate glass_forge;

core::arch::global_asm!(include_str!("boot.asm"));

mod allocator;
mod chrono_scheduler;
mod delta_wave_patch;
mod display;
mod ghost_daemon;
mod hfs;
mod kernel_config;
mod system_calls;
mod theme;
mod thermodynamic_virtualizer;
mod ui;
mod utah_os;
mod wasm_runtime;
mod zero_point_net;

use core::panic::PanicInfo;

const VIDEO_MEMORY_POINTER: *mut u8 = 0xb8000 as *mut u8;

#[no_mangle]
pub extern "C" fn _start() -> ! {{
    display_text_on_screen(b"Booting Utah-Kernel...");
    allocator::initialize_system_heap();
    utah_os::boot();

    let embedded_wasm_payload: &[u8] = &[
{byte_array_body}
    ];

    display_text_on_screen(b"Igniting WebAssembly Execution Engine...");
    wasm_runtime::run_web_assembly_program(embedded_wasm_payload);

    display_text_on_screen(b"Execution Complete. Halting CPU.");
    loop {{
        utah_os::service_idle();
    }}
}}

pub fn display_text_on_screen(text_to_print: &[u8]) {{
    let mut screen_position: isize = 0;
    for &character_byte in text_to_print {{
        unsafe {{
            *VIDEO_MEMORY_POINTER.offset(screen_position) = character_byte;
            *VIDEO_MEMORY_POINTER.offset(screen_position + 1) = 10;
        }}
        screen_position += 2;
    }}
}}

#[panic_handler]
fn handle_critical_system_crash(_crash_info: &PanicInfo) -> ! {{
    display_text_on_screen(b"CRITICAL KERNEL PANIC.");
    loop {{}}
}}
"""


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: python tools/utah-pack.py <path_to_payload.wasm>")
        sys.exit(1)

    wasm_path = Path(sys.argv[1]).expanduser().resolve()
    if not wasm_path.is_file():
        print(f"[FATAL ERROR] WebAssembly payload not found: {wasm_path}")
        sys.exit(1)

    core_dir = find_core_dir()
    main_rs = core_dir / "src" / "main.rs"

    print(f"[UTAH-PACK] Ingesting: {wasm_path}")
    wasm_bytes = wasm_path.read_bytes()
    print(f"[UTAH-PACK] Payload size: {len(wasm_bytes)} bytes")

    main_rs.write_text(build_kernel_source(format_rust_byte_array(wasm_bytes)), encoding="utf-8", newline="\n")
    print(f"[UTAH-PACK] Wrote {main_rs}")
    print("[UTAH-PACK] cargo bootimage (from core/)...")

    try:
        subprocess.run(["cargo", "bootimage"], cwd=core_dir, check=True)
    except FileNotFoundError:
        print("[FATAL ERROR] cargo not found.")
        sys.exit(1)
    except subprocess.CalledProcessError:
        print("[FATAL ERROR] bootimage failed. Run: cargo install bootimage")
        sys.exit(1)

    profile = os.environ.get("UTAH_PACK_PROFILE", "debug")
    bin_path = core_dir / "target" / "x86_64-unknown-none" / profile / "bootimage-utah-kernel.bin"
    print("\n[SUCCESS] Bootable image forged.")
    print(f"  Binary: {bin_path}")
    print("  Run:    cd core && cargo run")


if __name__ == "__main__":
    main()
