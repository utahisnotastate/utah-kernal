//! Zero-Point Network — resonance-frequency messaging without TCP/IP headers.
//! Production builds would DMA to/from the NIC TX/RX rings; this core is the logic layer.

extern crate alloc;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

/// A node is identified by vibrational frequency, not an IP address.
pub type ResonanceFrequency = u64;

/// Default frequency this kernel listens on until re-tuned.
pub const DEFAULT_LOCAL_RESONANCE: ResonanceFrequency = 12_345;

/// A raw, headerless block of intent. No TCP. No IP.
#[derive(Clone)]
pub struct IntentPayload {
    pub target_resonance: ResonanceFrequency,
    pub data: Vec<u8>,
}

pub struct ZeroPointNetwork {
    pub local_resonance: ResonanceFrequency,
    intent_buffer: Vec<IntentPayload>,
}

impl ZeroPointNetwork {
    pub fn new(frequency: ResonanceFrequency) -> Self {
        ZeroPointNetwork {
            local_resonance: frequency,
            intent_buffer: Vec::new(),
        }
    }

    /// Blasts data onto the physical medium (NIC TX ring in a full hardware build).
    pub fn broadcast_intent(&self, target: ResonanceFrequency, data: &[u8]) {
        // [PHYSICAL LAYER HOOK WOULD GO HERE]
        // Example: write_to_nic_dma(target, data);
        let _ = (target, data);
        crate::display_text_on_screen(b"[ZPN] Intent broadcasted to vacuum.");
    }

    /// Queues an incoming wave-state destined for this node's frequency.
    pub fn ingest_ambient_intent(&mut self, incoming: IntentPayload) {
        if incoming.target_resonance == self.local_resonance {
            self.intent_buffer.push(incoming);
        }
    }

    /// Returns the oldest intent payload waiting for this node.
    pub fn consume_intent(&mut self) -> Option<Vec<u8>> {
        if self.intent_buffer.is_empty() {
            return None;
        }
        let payload = self.intent_buffer.remove(0);
        Some(payload.data)
    }
}

lazy_static! {
    static ref GLOBAL_ZERO_POINT_NETWORK: Mutex<ZeroPointNetwork> =
        Mutex::new(ZeroPointNetwork::new(DEFAULT_LOCAL_RESONANCE));
    // Simulated ether: broadcast intents land here until a NIC driver drains/injects them.
    static ref GLOBAL_INTENT_ETHER: Mutex<Vec<IntentPayload>> = Mutex::new(Vec::new());
}

/// Broadcasts intent to the vacuum and loopbacks if this node matches `target`.
pub fn broadcast_intent_global(target: ResonanceFrequency, data: &[u8]) {
    let payload = IntentPayload {
        target_resonance: target,
        data: data.to_vec(),
    };

    {
        let mut network = GLOBAL_ZERO_POINT_NETWORK.lock();
        network.broadcast_intent(target, &payload.data);
        network.ingest_ambient_intent(payload.clone());
    }

    GLOBAL_INTENT_ETHER.lock().push(payload);
}

/// Delivers the next intent tuned to this node's local resonance frequency.
pub fn consume_intent_global() -> Option<Vec<u8>> {
    GLOBAL_ZERO_POINT_NETWORK.lock().consume_intent()
}

/// Retunes this node to a new telepathic mesh resonance frequency.
pub fn tune_local_resonance(frequency: ResonanceFrequency) {
    GLOBAL_ZERO_POINT_NETWORK.lock().local_resonance = frequency;
}

/// Returns the active local mesh frequency.
pub fn local_resonance_global() -> ResonanceFrequency {
    GLOBAL_ZERO_POINT_NETWORK.lock().local_resonance
}

/// Clears the simulated intent ether (non-essential RAM during ghost void transition).
pub fn drain_intent_ether_global() {
    GLOBAL_INTENT_ETHER.lock().clear();
}

/// Injects an intent as if it arrived from another node (testing / future RX hook).
#[allow(dead_code)]
pub fn ingest_ambient_intent_global(incoming: IntentPayload) {
    GLOBAL_ZERO_POINT_NETWORK
        .lock()
        .ingest_ambient_intent(incoming);
}
