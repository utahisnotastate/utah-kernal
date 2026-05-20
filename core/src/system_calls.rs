//! Secure "drive-through" between guest WebAssembly and the kernel.
//! Guests never touch hardware; they call small host functions we expose on purpose.

extern crate alloc;

use alloc::vec;
use wasmi::{Caller, Engine, Linker};

/// Largest single print the kernel will copy from guest memory (avoids huge allocations).
const MAX_PRINT_TEXT_BYTES: usize = 4096;
/// Largest blob the HFS will accept per save call.
const MAX_HOLOGRAM_BYTES: usize = 64 * 1024;
/// Largest intent the Zero-Point Network will accept per broadcast.
const MAX_BROADCAST_BYTES: usize = 64 * 1024;
const MAX_DELTA_BYTES: usize = 64 * 1024;
const MAX_GHOST_STATE_BYTES: usize = 256 * 1024;
const MAX_VIBE_THEME_BYTES: usize = 512;

/// Registers the kernel's safe "menu" of host functions on the linker before instantiation.
pub fn register_system_calls(linker: &mut Linker<()>, _engine: &Engine) {
    // System Call 1: print bytes from guest linear memory to the VGA text buffer.
    linker
        .func_wrap(
            "utah_system",
            "print_text_to_screen",
            |caller: Caller<'_, ()>, memory_pointer: i32, text_length: i32| {
                read_guest_bytes(&caller, memory_pointer, text_length, MAX_PRINT_TEXT_BYTES)
                    .map(|buffer| {
                        match core::str::from_utf8(&buffer) {
                            Ok(readable_text) => {
                                crate::display_text_on_screen(readable_text.as_bytes());
                            }
                            Err(_) => crate::display_text_on_screen(
                                b"[ERROR: Program tried to print invalid text characters]",
                            ),
                        }
                    })
                    .unwrap_or_else(|message| crate::display_text_on_screen(message));
            },
        )
        .expect("Failed to register the print system call.");

    // System Call 2: manifest (save) data into the Holographic File System; returns resonance signature.
    linker
        .func_wrap(
            "utah_system",
            "save_hologram",
            |caller: Caller<'_, ()>, memory_pointer: i32, data_length: i32| -> u64 {
                match read_guest_bytes(&caller, memory_pointer, data_length, MAX_HOLOGRAM_BYTES)
                {
                    Ok(buffer) => {
                        let signature = crate::hfs::manifest_data_global(&buffer);
                        crate::display_text_on_screen(
                            b"[HFS] Data etched. Resonance signature generated.",
                        );
                        signature
                    }
                    Err(message) => {
                        crate::display_text_on_screen(message);
                        0
                    }
                }
            },
        )
        .expect("Failed to register HFS save call.");

    // System Call 3: load hologram bytes into guest linear memory at destination_pointer.
    // Returns the number of bytes written (0 if not found or on error).
    linker
        .func_wrap(
            "utah_system",
            "load_hologram",
            |mut caller: Caller<'_, ()>, signature: u64, destination_pointer: i32| -> i32 {
                let Some(memory) = caller
                    .get_export("memory")
                    .and_then(|exported_item| exported_item.into_memory())
                else {
                    crate::display_text_on_screen(
                        b"[ERROR: WebAssembly program has no exported \"memory\"]",
                    );
                    return 0;
                };

                let Ok(offset) = usize::try_from(destination_pointer) else {
                    crate::display_text_on_screen(b"[ERROR: Invalid destination pointer]");
                    return 0;
                };

                let payload_length = match crate::hfs::hologram_length_global(signature) {
                    Some(length) => length,
                    None => {
                        crate::display_text_on_screen(b"[HFS] Resonance signature not found.");
                        return 0;
                    }
                };

                let mut staging_buffer = vec![0u8; payload_length];
                let Some(bytes_read) =
                    crate::hfs::retrieve_data_global(signature, &mut staging_buffer)
                else {
                    crate::display_text_on_screen(b"[HFS] Resonance signature not found.");
                    return 0;
                };

                if memory
                    .write(&mut caller, offset, &staging_buffer[..bytes_read])
                    .is_err()
                {
                    crate::display_text_on_screen(
                        b"[ERROR: Could not write hologram into guest memory]",
                    );
                    return 0;
                }

                crate::display_text_on_screen(
                    b"[HFS] Resonance match confirmed. Data injected.",
                );
                i32::try_from(bytes_read).unwrap_or(i32::MAX)
            },
        )
        .expect("Failed to register HFS load call.");

    // System Call 4: broadcast headerless intent to a target resonance frequency.
    linker
        .func_wrap(
            "utah_system",
            "broadcast",
            |caller: Caller<'_, ()>,
             target_freq: u64,
             memory_pointer: i32,
             data_length: i32| {
                match read_guest_bytes(
                    &caller,
                    memory_pointer,
                    data_length,
                    MAX_BROADCAST_BYTES,
                ) {
                    Ok(buffer) => {
                        crate::zero_point_net::broadcast_intent_global(target_freq, &buffer);
                    }
                    Err(message) => crate::display_text_on_screen(message),
                }
            },
        )
        .expect("Failed to register broadcast call.");

    // System Call 5: consume the next intent tuned to this node's local frequency.
    linker
        .func_wrap(
            "utah_system",
            "consume",
            |mut caller: Caller<'_, ()>, destination_pointer: i32| -> i32 {
                let Some(data) = crate::zero_point_net::consume_intent_global() else {
                    return 0;
                };

                let Some(memory) = caller
                    .get_export("memory")
                    .and_then(|exported_item| exported_item.into_memory())
                else {
                    crate::display_text_on_screen(
                        b"[ERROR: WebAssembly program has no exported \"memory\"]",
                    );
                    return 0;
                };

                let Ok(offset) = usize::try_from(destination_pointer) else {
                    crate::display_text_on_screen(b"[ERROR: Invalid destination pointer]");
                    return 0;
                };

                if memory.write(&mut caller, offset, &data).is_err() {
                    crate::display_text_on_screen(
                        b"[ERROR: Could not write intent into guest memory]",
                    );
                    return 0;
                }

                crate::display_text_on_screen(b"[ZPN] Telepathic state injected into guest.");
                i32::try_from(data.len()).unwrap_or(i32::MAX)
            },
        )
        .expect("Failed to register consume call.");

    // System Call 6: record user intent and return predicted next action (0 = none).
    linker
        .func_wrap(
            "utah_system",
            "record_and_predict",
            |_: Caller<'_, ()>, action_id: i32| -> u64 {
                let Ok(action) = u32::try_from(action_id) else {
                    crate::display_text_on_screen(b"[CHRONO] Invalid action id.");
                    return 0;
                };

                match crate::chrono_scheduler::record_and_predict_global(action) {
                    Some(predicted) => predicted as u64,
                    None => 0,
                }
            },
        )
        .expect("Failed to register record_and_predict call.");

    // System Call 7: consume a pre-staged predictive intent prepared by the scheduler.
    linker
        .func_wrap(
            "utah_system",
            "take_staged_intent",
            |_: Caller<'_, ()>| -> u64 {
                match crate::chrono_scheduler::take_staged_intent_global() {
                    Some(staged) => staged as u64,
                    None => 0,
                }
            },
        )
        .expect("Failed to register take_staged_intent call.");

    // System Call 8: read packed thermodynamic telemetry (idle_ticks:high, noise:low).
    linker
        .func_wrap(
            "utah_system",
            "read_thermodynamics",
            |_: Caller<'_, ()>| -> u64 {
                crate::thermodynamic_virtualizer::telemetry_snapshot_global()
            },
        )
        .expect("Failed to register read_thermodynamics call.");

    // System Call 9: retune local telepathic mesh frequency.
    linker
        .func_wrap(
            "utah_system",
            "tune_mesh",
            |_: Caller<'_, ()>, frequency: u64| {
                crate::zero_point_net::tune_local_resonance(frequency);
                crate::display_text_on_screen(b"[ZPN] Mesh resonance retuned.");
            },
        )
        .expect("Failed to register tune_mesh call.");

    // System Call 10: return active mesh resonance frequency.
    linker
        .func_wrap(
            "utah_system",
            "mesh_frequency",
            |_: Caller<'_, ()>| -> u64 {
                crate::zero_point_net::local_resonance_global()
            },
        )
        .expect("Failed to register mesh_frequency call.");

    // System Call 11: delta-wave patch (base + delta blobs in guest memory) -> HFS signature.
    linker
        .func_wrap(
            "utah_system",
            "apply_delta_patch",
            |caller: Caller<'_, ()>,
             base_pointer: i32,
             base_length: i32,
             delta_pointer: i32,
             delta_length: i32| -> u64 {
                let base = match read_guest_bytes(
                    &caller,
                    base_pointer,
                    base_length,
                    MAX_DELTA_BYTES,
                ) {
                    Ok(bytes) => bytes,
                    Err(message) => {
                        crate::display_text_on_screen(message);
                        return 0;
                    }
                };
                let delta = match read_guest_bytes(
                    &caller,
                    delta_pointer,
                    delta_length,
                    MAX_DELTA_BYTES,
                ) {
                    Ok(bytes) => bytes,
                    Err(message) => {
                        crate::display_text_on_screen(message);
                        return 0;
                    }
                };

                match crate::delta_wave_patch::commit_patched_image(&base, &delta) {
                    Ok(signature) => {
                        crate::display_text_on_screen(b"[DELTA] Atomic wave patch committed.");
                        signature
                    }
                    Err(()) => 0,
                }
            },
        )
        .expect("Failed to register apply_delta_patch call.");

    // System Call 12: ghost-daemon suspend — collapse guest state into HFS.
    linker
        .func_wrap(
            "utah_system",
            "ghost_suspend",
            |caller: Caller<'_, ()>, memory_pointer: i32, state_length: i32| -> u64 {
                match read_guest_bytes(
                    &caller,
                    memory_pointer,
                    state_length,
                    MAX_GHOST_STATE_BYTES,
                ) {
                    Ok(state) => crate::ghost_daemon::suspend_guest_state(&state),
                    Err(message) => {
                        crate::display_text_on_screen(message);
                        0
                    }
                }
            },
        )
        .expect("Failed to register ghost_suspend call.");

    // System Call 13: ghost-daemon resume — inject last collapsed state into guest memory.
    linker
        .func_wrap(
            "utah_system",
            "ghost_resume",
            |mut caller: Caller<'_, ()>, destination_pointer: i32| -> i32 {
                let Some(memory) = caller
                    .get_export("memory")
                    .and_then(|exported_item| exported_item.into_memory())
                else {
                    crate::display_text_on_screen(
                        b"[ERROR: WebAssembly program has no exported \"memory\"]",
                    );
                    return 0;
                };

                let Ok(offset) = usize::try_from(destination_pointer) else {
                    crate::display_text_on_screen(b"[ERROR: Invalid destination pointer]");
                    return 0;
                };

                let max_restore = 64 * 1024;
                let mut buffer = vec![0u8; max_restore];
                let Some(bytes_written) = crate::ghost_daemon::resume_guest_state(&mut buffer)
                else {
                    crate::display_text_on_screen(b"[GHOST] No suspended state available.");
                    return 0;
                };

                if memory
                    .write(&mut caller, offset, &buffer[..bytes_written])
                    .is_err()
                {
                    crate::display_text_on_screen(
                        b"[ERROR: Could not inject ghost state into guest memory]",
                    );
                    return 0;
                }

                crate::display_text_on_screen(b"[GHOST] State resumed from holographic matrix.");
                i32::try_from(bytes_written).unwrap_or(i32::MAX)
            },
        )
        .expect("Failed to register ghost_resume call.");

    // System Call 14: enter phantom void sleep — physical CPU halt (does not return).
    linker
        .func_wrap(
            "utah_system",
            "enter_phantom_sleep",
            |_: Caller<'_, ()>| -> u64 {
                crate::ghost_daemon::enter_phantom_sleep();
                #[allow(unreachable_code)]
                0
            },
        )
        .expect("Failed to register enter_phantom_sleep call.");

    // System Call 15: Glass-Forge — render a glass voxel node at (x, y) with intensity.
    linker
        .func_wrap(
            "utah_system",
            "render_interface_node",
            |_: Caller<'_, ()>, node_x: i32, node_y: i32, intensity: i32| {
                let level = u8::try_from(intensity.clamp(0, 255)).unwrap_or(128);
                crate::ui::render_interface_node(node_x, node_y, level);
            },
        )
        .expect("Failed to register render_interface_node call.");

    // System Call 16: register WASM linear memory for final ghost freeze.
    linker
        .func_wrap(
            "utah_system",
            "register_wasm_snapshot",
            |caller: Caller<'_, ()>, memory_pointer: i32, data_length: i32| {
                if let Ok(buffer) =
                    read_guest_bytes(&caller, memory_pointer, data_length, MAX_GHOST_STATE_BYTES)
                {
                    crate::ghost_daemon::register_wasm_linear_memory_snapshot(&buffer);
                }
            },
        )
        .expect("Failed to register register_wasm_snapshot call.");

    // System Call 17: final system freeze — HFS commit + cli/hlt (never returns).
    linker
        .func_wrap(
            "utah_system",
            "finalize_system_freeze",
            |_: Caller<'_, ()>| -> u64 {
                crate::ghost_daemon::finalize_system_freeze();
                #[allow(unreachable_code)]
                0
            },
        )
        .expect("Failed to register finalize_system_freeze call.");

    // System Call 18: dynamic voxel cloud at mouse/gaze vector (Glass-Forge).
    linker
        .func_wrap(
            "utah_system",
            "draw_voxel_cloud",
            |_: Caller<'_, ()>,
             origin_x: i32,
             origin_y: i32,
             vector_x: i32,
             vector_y: i32,
             intensity: i32| {
                let level = u8::try_from(intensity.clamp(0, 255)).unwrap_or(160);
                crate::ui::draw_dynamic_voxel_cloud(origin_x, origin_y, vector_x, vector_y, level);
            },
        )
        .expect("Failed to register draw_voxel_cloud call.");

    // System Call 19: set theme preset (0=dark, 1=golden, 2=light, 3=linda, 4=occult).
    linker
        .func_wrap(
            "utah_system",
            "set_theme_preset",
            |_: Caller<'_, ()>, preset_id: i32| {
                crate::theme::set_preset(crate::theme::ThemePreset::from_u32(
                    preset_id.max(0) as u32,
                ));
                crate::ui::render_boot_splash();
            },
        )
        .expect("Failed to register set_theme_preset call.");

    // System Call 20: vibe-code theme overrides from guest memory string.
    linker
        .func_wrap(
            "utah_system",
            "apply_vibe_theme",
            |caller: Caller<'_, ()>, memory_pointer: i32, text_length: i32| {
                if let Ok(intent) = read_guest_bytes(
                    &caller,
                    memory_pointer,
                    text_length,
                    MAX_VIBE_THEME_BYTES,
                ) {
                    if let Ok(text) = core::str::from_utf8(&intent) {
                        crate::theme::apply_vibe_modification(text);
                        crate::ui::render_boot_splash();
                    }
                }
            },
        )
        .expect("Failed to register apply_vibe_theme call.");

    // System Call 21: unified virtual canvas dimensions (high 32 = width, low 32 = height).
    linker
        .func_wrap(
            "utah_system",
            "get_canvas_dimensions",
            |_: Caller<'_, ()>| -> i64 {
                let (w, h) = crate::display::global_canvas_dimensions();
                (((w as u64) << 32) | (h as u64)) as i64
            },
        )
        .expect("Failed to register get_canvas_dimensions call.");

    // System Call 22: pin window to monitor index (rule 0 = strict index).
    linker
        .func_wrap(
            "utah_system",
            "pin_window_to_monitor",
            |_: Caller<'_, ()>, monitor_index: i32, width: i32, height: i32| {
                let w = width.max(0) as u32;
                let h = height.max(0) as u32;
                let idx = monitor_index.max(0) as u32;
                crate::display::pin_window(
                    "WASM-Pinned-Window",
                    w,
                    h,
                    crate::display::ApplicationPinRule::StrictMonitorIndex(idx),
                );
                crate::display::composite_primary_head_to_framebuffer();
            },
        )
        .expect("Failed to register pin_window_to_monitor call.");

    // System Call 23: resolve global (x,y) -> packed (monitor:16, local_x:16, local_y:32) or 0.
    linker
        .func_wrap(
            "utah_system",
            "resolve_global_pixel",
            |_: Caller<'_, ()>, global_x: i32, global_y: i32| -> i64 {
                let topo = crate::display::topology();
                if let Some((head, lx, ly)) = topo.resolve_physical_coordinates(global_x, global_y)
                {
                    let monitor = head.monitor_hardware_index as u64;
                    ((monitor << 48) | ((lx as u64) << 32) | (ly as u64)) as i64
                } else {
                    0
                }
            },
        )
        .expect("Failed to register resolve_global_pixel call.");

    // System Call 24: refresh pinned window borders on all heads.
    linker
        .func_wrap(
            "utah_system",
            "refresh_display_pins",
            |_: Caller<'_, ()>| {
                crate::display::render_pinned_windows();
                crate::display::composite_primary_head_to_framebuffer();
            },
        )
        .expect("Failed to register refresh_display_pins call.");
}

fn read_guest_bytes(
    caller: &Caller<'_, ()>,
    memory_pointer: i32,
    data_length: i32,
    max_bytes: usize,
) -> Result<alloc::vec::Vec<u8>, &'static [u8]> {
    let Some(memory) = caller
        .get_export("memory")
        .and_then(|exported_item| exported_item.into_memory())
    else {
        return Err(b"[ERROR: WebAssembly program has no exported \"memory\"]");
    };

    let Ok(length) = usize::try_from(data_length) else {
        return Err(b"[ERROR: Negative buffer length]");
    };
    if length == 0 {
        return Ok(vec![]);
    }

    let safe_length = length.min(max_bytes);
    let Ok(offset) = usize::try_from(memory_pointer) else {
        return Err(b"[ERROR: Invalid memory pointer]");
    };

    let mut buffer = vec![0u8; safe_length];
    if memory.read(caller, offset, &mut buffer).is_err() {
        return Err(b"[ERROR: Could not read guest memory]");
    }
    Ok(buffer)
}
