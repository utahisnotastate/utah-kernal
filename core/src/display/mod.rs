//! Utah-OS display subsystem — unified topology, EDID optimization, intent pinning.

mod edid;
mod topology;
mod window_manager;

pub use edid::MonitorCapabilitiesProfile;
pub use topology::GlobalDisplayTopology;
pub use window_manager::{ApplicationPinRule, StructuralWindowFrame, WindowPinRegistry};

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref GLOBAL_TOPOLOGY: Mutex<GlobalDisplayTopology> = Mutex::new(GlobalDisplayTopology::new());
    static ref WINDOW_REGISTRY: Mutex<WindowPinRegistry> = Mutex::new(WindowPinRegistry::new());
}

/// Returns the global unified display topology.
pub fn topology() -> spin::MutexGuard<'static, GlobalDisplayTopology> {
    GLOBAL_TOPOLOGY.lock()
}

/// Probes EDID (simulated until GOP/DDC), registers heads, pins demo windows.
pub fn initialize_unified_topology() {
    let mut topo = GLOBAL_TOPOLOGY.lock();

    let blank_edid: [u8; 128] = [0; 128];
    let edid_primary = MonitorCapabilitiesProfile::parse_and_optimize_edid(&blank_edid);
    let edid_secondary = MonitorCapabilitiesProfile::simulated_secondary();

    // Primary head — gaming panel (QEMU-safe resolution; GOP maps full 1920x1080 in production)
    topo.register_monitor_head(800, 600, 800, edid_primary.optimized_refresh_rate_ceiling_hz);

    // Secondary head — auxiliary panel, placed to the right on unified canvas
    topo.register_monitor_head(640, 480, 640, edid_secondary.optimized_refresh_rate_ceiling_hz);

    crate::display_text_on_screen(b"[DISPLAY] Unified virtual canvas online.");
}

/// Applies sovereign EDID refresh ceilings to all registered heads (already set at register).
pub fn apply_edid_overrides() {
    let _topo = GLOBAL_TOPOLOGY.lock();
    crate::display_text_on_screen(b"[DISPLAY] Sovereign EDID overrides applied.");
}

/// Registers default pinned application frames (browser on monitor 1, AI on fastest Hz).
pub fn register_default_pinned_windows() {
    let mut registry = WINDOW_REGISTRY.lock();
    registry.clear();

    registry.register(StructuralWindowFrame::new(
        "Utah-Browser-Core",
        400,
        300,
        ApplicationPinRule::StrictMonitorIndex(1),
    ));
    registry.register(StructuralWindowFrame::new(
        "Sovereign-Prime-AI",
        320,
        240,
        ApplicationPinRule::HighestPerformanceDisplay,
    ));
    registry.register(StructuralWindowFrame::new(
        "HFS-Explorer",
        360,
        280,
        ApplicationPinRule::StrictMonitorIndex(0),
    ));
}

/// Places and draws all pinned windows on the unified canvas.
pub fn render_pinned_windows() {
    let topo = GLOBAL_TOPOLOGY.lock();
    let mut registry = WINDOW_REGISTRY.lock();
    registry.place_all(&topo);

    let cyan = glass_forge::ColorRgb {
        red: 56,
        green: 189,
        blue: 248,
    };
    let gold = glass_forge::ColorRgb {
        red: 234,
        green: 179,
        blue: 8,
    };

    registry.frames[0].commit_window_draw_pulses(&topo, cyan);
    if registry.frames.len() > 1 {
        registry.frames[1].commit_window_draw_pulses(&topo, gold);
    }
    if registry.frames.len() > 2 {
        registry.frames[2].commit_window_draw_pulses(&topo, cyan);
    }

    crate::display_text_on_screen(b"[DISPLAY] Intent-pinned windows committed.");
}

/// Full display stack init for boot.
pub fn boot_display_stack() {
    initialize_unified_topology();
    apply_edid_overrides();
    register_default_pinned_windows();
    render_pinned_windows();
}

/// Pin a window by rule (host / future config API).
pub fn pin_window(title: &'static str, width: u32, height: u32, rule: ApplicationPinRule) {
    let mut registry = WINDOW_REGISTRY.lock();
    let mut frame = StructuralWindowFrame::new(title, width, height, rule);
    let topo = GLOBAL_TOPOLOGY.lock();
    frame.calculate_automated_placement(&topo);
    frame.commit_window_draw_pulses(
        &topo,
        glass_forge::ColorRgb {
            red: 200,
            green: 120,
            blue: 255,
        },
    );
    registry.register(frame);
}

/// Global canvas size for unified pointer routing (future HID).
pub fn global_canvas_dimensions() -> (u32, u32) {
    let topo = GLOBAL_TOPOLOGY.lock();
    (topo.combined_virtual_width, topo.combined_virtual_height)
}

/// Blits monitor 0 RGB pixels into the Glass-Forge BGRA framebuffer (QEMU-visible).
pub fn composite_primary_head_to_framebuffer() {
    let topo = GLOBAL_TOPOLOGY.lock();
    let Some(head) = topo.attached_monitor_heads.first() else {
        return;
    };
    let w = head.resolution_width_pixels.min(glass_forge::FRAMEBUFFER_WIDTH as u32) as i32;
    let h = head
        .resolution_height_pixels
        .min(glass_forge::FRAMEBUFFER_HEIGHT as u32) as i32;
    let stride = head.pixels_per_scan_line.max(head.resolution_width_pixels);

    for y in 0..h {
        for x in 0..w {
            let offset = (y as u32 * stride + x as u32) as isize;
            let packed = unsafe {
                if head.video_memory_base_address.is_null() {
                    continue;
                }
                head.video_memory_base_address
                    .offset(offset)
                    .read_volatile()
            };
            let r = ((packed >> 16) & 0xFF) as u8;
            let g = ((packed >> 8) & 0xFF) as u8;
            let b = (packed & 0xFF) as u8;
            glass_forge::Framebuffer::new().write_pixel(x, y, r, g, b, 255);
        }
    }
}
