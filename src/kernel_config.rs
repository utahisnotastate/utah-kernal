//! Utah-OS master configuration — unified state-machine parameters for all subsystems.

use crate::zero_point_net::ResonanceFrequency;

/// Schumann-resonance-scale mesh identifier (7.83 Hz encoded as master tuning constant).
pub const SCHUMANN_RESONANCE_FREQUENCY: ResonanceFrequency = 7_830_000_000;

/// The "Omega" configuration: one operating system as a single unified state machine.
#[derive(Clone, Copy, Debug)]
pub struct OmegaConfiguration {
    /// Enables the Chrono-Scheduler probability manifold.
    pub temporal_sequencing_enabled: bool,
    /// Telepathic mesh resonance ID for this node.
    pub resonant_network_frequency: ResonanceFrequency,
    /// Enables thermodynamic idle harvesting (heat-as-compute model).
    pub thermodynamic_cooling_mode: bool,
}

pub const UTAH_OS_MASTER_CONFIG: OmegaConfiguration = OmegaConfiguration {
    temporal_sequencing_enabled: true,
    resonant_network_frequency: SCHUMANN_RESONANCE_FREQUENCY,
    thermodynamic_cooling_mode: true,
};

/// Applies master configuration to network, scheduler, and virtualizer subsystems.
pub fn apply_master_configuration() {
    crate::zero_point_net::tune_local_resonance(UTAH_OS_MASTER_CONFIG.resonant_network_frequency);
    crate::thermodynamic_virtualizer::configure(UTAH_OS_MASTER_CONFIG.thermodynamic_cooling_mode);
    crate::ghost_daemon::initialize();
}

/// Whether predictive temporal sequencing is active.
pub fn temporal_sequencing_enabled() -> bool {
    UTAH_OS_MASTER_CONFIG.temporal_sequencing_enabled
}
