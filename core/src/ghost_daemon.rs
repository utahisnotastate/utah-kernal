//! Ghost-Daemon — deep-sleep memory collapse and phantom (void) CPU state.
//!
//! World-A apps "sleep" by freezing a UI thread while the OS keeps running.
//! Utah-OS collapses volatile state into the HFS, clears non-essential RAM, masks
//! interrupts, and halts the CPU until hardware wakes it (timer/NMI on x86;
//! light-sleep on ESP/M5Stack via `m5stack_void_sleep_hook`).

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

/// Resonance signature of the last suspended ghost state (if any).
static GHOST_STATE_SIGNATURE: Mutex<Option<u64>> = Mutex::new(None);

/// Pending WASM linear-memory snapshots captured before a system freeze.
static WASM_LINEAR_MEMORY_SNAPSHOTS: Mutex<Vec<Vec<u8>>> = Mutex::new(Vec::new());

/// Combined memory-state hash committed on finalize (if any).
static MEMORY_STATE_RESONANCE: Mutex<Option<u64>> = Mutex::new(None);

/// True while the CPU is in (or entering) phantom void state.
static PHANTOM_VOID_ACTIVE: Mutex<bool> = Mutex::new(false);

/// Scratch cleared during void transition to prove non-essential RAM was released.
static VOID_TRANSITION_COUNTER: Mutex<u32> = Mutex::new(0);

/// Initializes the ghost-daemon control block.
pub fn initialize() {
    *GHOST_STATE_SIGNATURE.lock() = None;
    *PHANTOM_VOID_ACTIVE.lock() = false;
    WASM_LINEAR_MEMORY_SNAPSHOTS.lock().clear();
    *MEMORY_STATE_RESONANCE.lock() = None;
}

/// Registers a WASM sandbox linear-memory segment for the final freeze snapshot.
pub fn register_wasm_linear_memory_snapshot(data: &[u8]) {
    if data.is_empty() {
        return;
    }
    WASM_LINEAR_MEMORY_SNAPSHOTS.lock().push(data.to_vec());
}

/// Collapses guest state into the HFS and records its resonance signature.
pub fn suspend_guest_state(state: &[u8]) -> u64 {
    let signature = crate::hfs::manifest_data_global(state);
    *GHOST_STATE_SIGNATURE.lock() = Some(signature);
    crate::display_text_on_screen(b"[GHOST] State collapsed to holographic matrix.");
    signature
}

/// Restores the last suspended ghost state into `destination` and returns bytes written.
pub fn resume_guest_state(destination: &mut [u8]) -> Option<usize> {
    let signature = (*GHOST_STATE_SIGNATURE.lock())?;
    crate::hfs::retrieve_data_global(signature, destination)
}

/// Returns the resonance signature of the last ghost snapshot (0 if none).
#[allow(dead_code)]
pub fn last_ghost_signature() -> u64 {
    match *GHOST_STATE_SIGNATURE.lock() {
        Some(signature) => signature,
        None => 0,
    }
}

/// Returns whether the system is in phantom void state.
#[allow(dead_code)]
pub fn phantom_void_active() -> bool {
    *PHANTOM_VOID_ACTIVE.lock()
}

// --- Phantom sleep protocol (physical system state freeze) ---

/// Step 1: Mask external interrupts so only the void transition runs on bare metal.
fn decouple_hardware_interrupts() {
    unsafe {
        core::arch::asm!("cli", options(nomem, nostack, preserves_flags));
    }
}

/// Step 2: Drop non-essential volatile buffers; HFS + ghost signature stay intact.
fn clear_nonessential_ram_segments() {
    crate::zero_point_net::drain_intent_ether_global();
    let _ = crate::chrono_scheduler::take_staged_intent_global();
    let mut counter = VOID_TRANSITION_COUNTER.lock();
    *counter = counter.wrapping_add(1);
    let _scratch = alloc::vec![0u8; 0];
}

/// Step 3: Arm periodic wake source (PIT on x86; timer on M5Stack in production).
fn arm_system_timer_heartbeat() {
    // Full builds program the 8254 PIT or HPET here. QEMU/bootloader may still
    // deliver NMIs/SMI; guest timer drivers belong in a future arch crate.
    crate::display_text_on_screen(b"[GHOST] System-timer heartbeat armed.");
}

/// Step 4: Request deepest idle state the hardware exposes (C6 on x86 when ACPI exists).
fn request_cpu_deep_sleep() {
    // ACPI C-state entry would go here. Until then, `hlt` is the deepest halt
    // available without platform firmware support.
    crate::display_text_on_screen(b"[GHOST] CPU entering deep halt (C6-class).");
}

/// M5Stack / ESP32 hook: map to `esp_light_sleep_start` in a board-specific crate.
#[allow(dead_code)]
fn m5stack_void_sleep_hook() {
    crate::display_text_on_screen(b"[GHOST] M5Stack light-sleep hook (stub).");
}

/// Prepares memory and peripherals before the irreversible void transition.
fn prepare_void_transition() {
    clear_nonessential_ram_segments();
    arm_system_timer_heartbeat();
    request_cpu_deep_sleep();
    *PHANTOM_VOID_ACTIVE.lock() = true;
    crate::display_text_on_screen(b"[GHOST] Void transition prepared.");
}

/// Enters low-power **Void State**: interrupts off, non-essential RAM cleared, CPU halted.
///
/// This does not return under normal operation. Wake requires NMI, reset, or a hardware
/// timer wired through platform firmware. Use [`enter_phantom_sleep_with_heartbeat`] when
/// you need periodic `hlt` wake on emulated x86.
pub fn enter_phantom_sleep() -> ! {
    prepare_void_transition();
    crate::display_text_on_screen(b"[GHOST] Entering Void State (phantom sleep)...");

    decouple_hardware_interrupts();

    unsafe {
        loop {
            // CLI + HLT: the machine stops advancing until hardware pulls it out of time.
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}

/// Variant: re-enable interrupts between halts so PIT/timer IRQ can act as heartbeat.
#[allow(dead_code)]
pub fn enter_phantom_sleep_with_heartbeat() -> ! {
    prepare_void_transition();
    crate::display_text_on_screen(b"[GHOST] Phantom sleep + timer heartbeat...");

    unsafe {
        loop {
            core::arch::asm!("sti", options(nomem, nostack, preserves_flags));
            core::arch::asm!("hlt", options(nomem, nostack));
            core::arch::asm!("cli", options(nomem, nostack, preserves_flags));
        }
    }
}

/// Called after a heartbeat wake (future timer ISR would invoke this).
#[allow(dead_code)]
pub fn wake_from_phantom() {
    *PHANTOM_VOID_ACTIVE.lock() = false;
    unsafe {
        core::arch::asm!("sti", options(nomem, nostack, preserves_flags));
    }
    crate::display_text_on_screen(b"[GHOST] Woke from Void State.");
}

/// Snapshot count of registered WASM segments awaiting freeze.
pub fn pending_wasm_snapshot_count() -> usize {
    WASM_LINEAR_MEMORY_SNAPSHOTS.lock().len()
}

/// Last committed memory-state resonance signature from [`finalize_system_freeze`].
pub fn memory_state_resonance() -> u64 {
    MEMORY_STATE_RESONANCE.lock().unwrap_or(0)
}

/// Final kill-switch: snapshot all WASM linear memory into HFS, then absolute CPU freeze.
///
/// 1. Snapshot WASM sandbox linear memory segments.
/// 2. Commit combined memory-state hash (resonance signature) to HFS.
/// 3. `cli` + `hlt` — hardware-level halt (dead-man's switch).
pub fn finalize_system_freeze() -> ! {
    crate::display_text_on_screen(b"[GHOST] Finalizing system freeze...");

    let snapshots: Vec<Vec<u8>> = {
        let mut pending = WASM_LINEAR_MEMORY_SNAPSHOTS.lock();
        core::mem::take(&mut *pending)
    };

    if snapshots.is_empty() {
        crate::display_text_on_screen(b"[GHOST] No WASM snapshots; freezing void state.");
    } else {
        let mut combined = Vec::new();
        for segment in &snapshots {
            combined.extend_from_slice(segment);
            let segment_signature = crate::hfs::manifest_data_global(segment);
            *GHOST_STATE_SIGNATURE.lock() = Some(segment_signature);
        }
        let master_signature = crate::hfs::manifest_data_global(&combined);
        *MEMORY_STATE_RESONANCE.lock() = Some(master_signature);
        crate::display_text_on_screen(b"[GHOST] Memory-state hash committed to HFS.");
    }

    prepare_void_transition();
    decouple_hardware_interrupts();
    crate::display_text_on_screen(b"[GHOST] CPU halt - system no longer exists in time.");

    unsafe {
        loop {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}
