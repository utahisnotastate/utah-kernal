//! Utah-OS (UTA H-OS) — master boot orchestration for the Omega state-space console.

/// Utah-OS product version string (also shown on boot banner).
#[allow(dead_code)]
pub const UTAH_OS_VERSION: &str = "0.5.0-omega";

/// Initializes all Utah-OS subsystems from the master configuration.
pub fn boot() {
    crate::display_text_on_screen(b"Utah-OS v0.5.0-omega booting...");
    crate::kernel_config::apply_master_configuration();
    crate::display_text_on_screen(b"Telepathic mesh tuned.");
    crate::display_text_on_screen(b"Chrono-Scheduler manifold active.");
    crate::thermodynamic_virtualizer::bootstrap();
    crate::display_text_on_screen(b"Thermodynamic virtualizer online.");
    crate::display_text_on_screen(b"Ghost-Daemon armed.");
    crate::ui::init_framebuffer();
    crate::ui::render_boot_splash();
    crate::display_text_on_screen(b"Glass-Forge interface manifested.");
}

/// Idle servicing hook — harvest thermodynamics and run ghost housekeeping.
pub fn service_idle() {
    crate::thermodynamic_virtualizer::harvest_idle_cycle_global();
}
