//! # Glass-Forge
//! Direct-to-VRAM holographic UI for Utah-OS — no X11, Wayland, HTML, or Qt.

#![no_std]

mod framebuffer;
mod glass;
mod manifold;
mod theme;
mod theme_runtime;
mod voxel;

pub use framebuffer::{
    clear_framebuffer, map_physical_framebuffer, Framebuffer, FramebufferConfig,
    FRAMEBUFFER_HEIGHT, FRAMEBUFFER_WIDTH,
};
pub use glass::{
    draw_boot_splash, draw_glass_panel, draw_particle_node, render_interface_node,
};
pub use manifold::render_total_os_manifold;
pub use theme::{ColorRgb, SystemTheme, ThemePreset};
pub use theme_runtime::{
    active_theme, apply_vibe_modification, render_desktop, set_preset,
};
pub use voxel::draw_dynamic_voxel_cloud;

use core::sync::atomic::{AtomicBool, Ordering};

static FRAMEBUFFER_READY: AtomicBool = AtomicBool::new(false);

/// Initializes the linear framebuffer with the active theme background.
pub fn init_framebuffer() {
    let theme = theme_runtime::active_theme();
    clear_framebuffer(theme.background.red, theme.background.green, theme.background.blue);
    FRAMEBUFFER_READY.store(true, Ordering::SeqCst);
}

pub fn framebuffer_ready() -> bool {
    FRAMEBUFFER_READY.load(Ordering::SeqCst)
}

/// Renders the full Utah-OS desktop manifold (primary visual mode).
pub fn render_boot_splash() {
    if !framebuffer_ready() {
        init_framebuffer();
    }
    theme_runtime::render_desktop();
}
