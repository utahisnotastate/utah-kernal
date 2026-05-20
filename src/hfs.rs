//! Holographic File System — content-addressable in-kernel storage keyed by data hash.

extern crate alloc;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

/// Implements a bare-metal DJB2 hashing algorithm to calculate
/// the structural resonance of raw binary data.
fn calculate_resonance_signature(data: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    for &byte in data {
        // hash * 33 + c
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

/// A stored blob has no path or filename — only its content hash (resonance signature).
pub struct Hologram {
    pub resonance_signature: u64,
    pub payload: Vec<u8>,
}

pub struct HolographicFileSystem {
    matrix: Vec<Hologram>,
}

impl HolographicFileSystem {
    pub fn new() -> Self {
        HolographicFileSystem {
            matrix: Vec::new(),
        }
    }

    /// Ingests raw data, computes its resonance signature, and stores it if not already present.
    pub fn manifest_data(&mut self, data: &[u8]) -> u64 {
        let signature = calculate_resonance_signature(data);

        // Absolute deduplication: identical content shares one slot in the matrix.
        if self.retrieve_data(signature).is_none() {
            self.matrix.push(Hologram {
                resonance_signature: signature,
                payload: data.to_vec(),
            });
        }

        signature
    }

    /// Returns a view of stored bytes for the given resonance signature, if any.
    pub fn retrieve_data(&self, signature: u64) -> Option<&[u8]> {
        for holo in &self.matrix {
            if holo.resonance_signature == signature {
                return Some(&holo.payload);
            }
        }
        None
    }
}

lazy_static! {
    static ref GLOBAL_HOLOGRAPHIC_MATRIX: Mutex<HolographicFileSystem> =
        Mutex::new(HolographicFileSystem::new());
}

/// Stores data in the global matrix and returns its resonance signature.
pub fn manifest_data_global(data: &[u8]) -> u64 {
    GLOBAL_HOLOGRAPHIC_MATRIX.lock().manifest_data(data)
}

/// Copies stored data for a signature into `out` and returns the number of bytes written.
pub fn retrieve_data_global(signature: u64, out: &mut [u8]) -> Option<usize> {
    let matrix = GLOBAL_HOLOGRAPHIC_MATRIX.lock();
    let payload = matrix.retrieve_data(signature)?;
    let length = payload.len().min(out.len());
    out[..length].copy_from_slice(&payload[..length]);
    Some(length)
}

/// Returns the full byte length of a stored hologram (for sizing guest buffers).
pub fn hologram_length_global(signature: u64) -> Option<usize> {
    GLOBAL_HOLOGRAPHIC_MATRIX
        .lock()
        .retrieve_data(signature)
        .map(|data| data.len())
}
