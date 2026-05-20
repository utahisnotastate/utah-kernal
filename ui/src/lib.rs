//! # Glass-Forge
//! Direct-to-VRAM holographic UI for Utah-OS — no X11, Wayland, HTML, or Qt.

#![no_std]

mod framebuffer;
mod glass;

pub use framebuffer::{
    clear_framebuffer, map_physical_framebuffer, Framebuffer, FramebufferConfig,
    FRAMEBUFFER_HEIGHT, FRAMEBUFFER_WIDTH,
};
pub use glass::{
    draw_boot_splash, draw_glass_panel, draw_particle_node, render_interface_node,
};

use core::sync::atomic::{AtomicBool, Ordering};

static FRAMEBUFFER_READY: AtomicBool = AtomicBool::new(false);

/// Initializes the linear framebuffer canvas.
pub fn init_framebuffer() {
    clear_framebuffer(0x08, 0x0C, 0x18);
    FRAMEBUFFER_READY.store(true, Ordering::SeqCst);
}

pub fn framebuffer_ready() -> bool {
    FRAMEBUFFER_READY.load(Ordering::SeqCst)
}

/// Renders the Utah-OS Glass-Forge boot splash.
pub fn render_boot_splash() {
    if !framebuffer_ready() {
        init_framebuffer();
    }
    draw_boot_splash();
}
