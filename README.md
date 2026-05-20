# Utah-Kernel / Utah-OS: Ring-0 WebAssembly Unikernel Architecture

> **Utah-OS** is the product console layered on Utah-Kernel. See [UTAH_OS.md](UTAH_OS.md) for the SOTA feature matrix and [MONETIZATION.md](MONETIZATION.md) for the commercial blueprint.

## Abstract

The Utah-Kernel is an experimental, bare-metal unikernel designed to reduce the thermodynamic and computational overhead of traditional hardware context switching. By running guest logic as WebAssembly on bare metal (with host functions for hardware access), the kernel targets near-zero latency execution paths compared with conventional user/kernel mode transitions. Security and memory isolation are enforced through WebAssembly validation and sandboxed linear memory, complemented by explicit host function boundaries in `src/system_calls.rs`.

## Architectural Advantages

1. **Zero Context Switching (design goal):** Applications compile to WebAssembly and run in a single address space with kernel services exposed as host imports.
2. **Mathematical Sandboxing:** Module validation and typed linear memory reduce common memory-safety failure modes before code runs on hardware.
3. **Single Address Space:** Avoids per-process page-table churn and TLB shootdown patterns typical of multi-process POSIX systems.
4. **Language-Agnostic Ecosystem:** Any toolchain that emits `wasm32` modules (Rust, C, C++, Go, and others) can supply a payload for packaging.

## Prerequisites

To compile the Utah-Kernel and forge bootable images, install:

- [rustup](https://rustup.rs/) with the **nightly** toolchain (see `rust-toolchain.toml` in this repo)
- [cargo-bootimage](https://github.com/rust-osdev/bootimage) for bootable `.bin` images
- [QEMU](https://www.qemu.org/) (`qemu-system-x86_64`) for emulation

```bash
rustup toolchain install nightly
rustup component add rust-src llvm-tools-preview --toolchain nightly
rustup target add x86_64-unknown-none --toolchain nightly
cargo install bootimage
```

On Windows, add QEMU to your `PATH` after installation.

## Compilation and Deployment

The repository includes **`utah-pack.py`**, a build orchestrator that embeds your compiled `.wasm` payload into `src/main.rs` and invokes `cargo bootimage`.

1. Build your application to WebAssembly (`.wasm`). The guest should export `_start` and linear memory named `memory`. To print to the screen, import `(utah_system, print_text_to_screen)` with two `i32` parameters (pointer, length) as defined in `src/system_calls.rs`.
2. Package and compile the kernel:

```bash
python utah-pack.py path/to/your_application.wasm
```

On Windows, if `python` is not on your PATH, use:

```bash
py -3 utah-pack.py path\to\your_application.wasm
```

3. Boot the image in QEMU:

```bash
cargo run
```

Optional: build a release image with:

```bash
set UTAH_PACK_PROFILE=release
python utah-pack.py path/to/your_application.wasm
cargo bootimage --release
```

Output (debug build) is typically:

`target/x86_64-unknown-none/debug/bootimage-utah-kernel.bin`

## Repository Layout

| Path | Purpose |
|------|---------|
| `src/main.rs` | Kernel entry, VGA text output, payload hook (rewritten by `utah-pack.py`) |
| `src/boot.asm` | Multiboot2 header for GRUB-compatible boot loaders |
| `src/allocator.rs` | Heap allocator for guest/kernel allocations |
| `src/wasm_runtime.rs` | Wasmi loader and instance startup |
| `src/system_calls.rs` | Host functions (system calls) exposed to guests |
| `src/hfs.rs` | Holographic File System — content-addressable, deduplicated in-kernel storage |
| `src/zero_point_net.rs` | Zero-Point Network — resonance-frequency intent broadcast/consume |
| `src/chrono_scheduler.rs` | Chrono-Scheduler — predictive intent pre-staging |
| `utah-deploy.sh` | Release forge script (bootimage + optional AES packaging) |
| `src/kernel_config.rs` | Utah-OS `OmegaConfiguration` master constants |
| `src/utah_os.rs` | Utah-OS boot orchestration and idle service loop |
| `src/thermodynamic_virtualizer.rs` | Idle harvest + thermodynamic telemetry |
| `src/delta_wave_patch.rs` | Delta-wave in-place patching |
| `src/ghost_daemon.rs` | Ghost-Daemon suspend/resume via HFS |
| `UTAH_OS.md` | Utah-OS architecture and host API index |
| `MONETIZATION.md` | The Governor — Compute Sovereignty business model |
| `utah-pack.py` | WASM ingest, source injection, `cargo bootimage` driver |

## Extensibility and System Calls

Hardware and OS services are exposed to WebAssembly through **host functions** registered in `src/system_calls.rs`. New capabilities (networking, timers) should be added there so guests never touch device registers directly.

### Holographic File System (HFS)

Guest modules can persist blobs by **content hash** (DJB2 resonance signature), not by file paths:

| Host import | Parameters | Returns |
|-------------|------------|---------|
| `utah_system::save_hologram` | `(pointer: i32, length: i32)` | `i64` resonance signature |
| `utah_system::load_hologram` | `(signature: i64, dest_pointer: i32)` | `i32` bytes written |

Identical payloads deduplicate automatically. Storage lives in RAM until a block-device driver exists.

### Zero-Point Network (ZPN)

Headerless intent messaging by **resonance frequency** (default local tune: `12345`):

| Host import | Parameters | Returns |
|-------------|------------|---------|
| `utah_system::broadcast` | `(target_freq: i64, pointer: i32, length: i32)` | — |
| `utah_system::consume` | `(dest_pointer: i32)` | `i32` bytes written |

Broadcasts loop back when `target_freq` matches this node's local frequency. A global intent ether buffer holds payloads until a physical NIC driver is wired in.

### Chrono-Scheduler (predictive intent)

| Host import | Parameters | Returns |
|-------------|------------|---------|
| `utah_system::record_and_predict` | `(action_id: i32)` | `i64` predicted next action (0 = none) |
| `utah_system::take_staged_intent` | `()` | `i64` pre-staged action id (0 = none) |

Built-in transitions include `1→2→3→4` and `10→11→12`. When a prediction fires, the kernel pre-stages a small allocator warm-up and queues the intent for `take_staged_intent`.

### Production deploy

```bash
./utah-deploy.sh
```

Requires `cargo`, `bootimage`, and optionally `openssl` for `utah_v1_signed.pkg`.

### Ghost-Daemon phantom sleep

`enter_phantom_sleep()` collapses volatile buffers, masks interrupts, and executes `cli` + `hlt` — a full CPU void state, not a frozen UI thread. Exposed to WASM as `utah_system::enter_phantom_sleep` (diverges). Use `enter_phantom_sleep_with_heartbeat` in-kernel when PIT/timer wake is wired.

## Documentation (by audience)

| Tier | File | Audience |
|------|------|----------|
| 1 | [README.md](README.md) | Engineers and contributors |
| 2 | [CIVILIAN_DOCUMENTATION.md](CIVILIAN_DOCUMENTATION.md) | Non-technical readers |
| 3 | [CHILD_MANUAL.md](CHILD_MANUAL.md) | Young learners and beginners |
| — | [UTAH_OS.md](UTAH_OS.md) | Utah-OS SOTA matrix and module guide |
| — | [MONETIZATION.md](MONETIZATION.md) | Commercial / fleet monetization blueprint |

## Status

The kernel is a **high-density prototype**: bootloader integration, heap, Wasmi runtime, and a print host function are in place. NVMe/SATA, TCP/IP, and automatic on-disk WASM discovery are not yet implemented. For production-style workflows, use `utah-pack.py` instead of hand-editing `main.rs`.

## License

Specify your license before a public GitHub release (e.g. MIT or Apache-2.0).
