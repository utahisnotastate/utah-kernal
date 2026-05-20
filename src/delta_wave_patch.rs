//! Delta-Wave atomic in-place patching — apply binary deltas without full reinstall.

extern crate alloc;

use alloc::vec::Vec;

/// Applies a simple XOR delta stream to `base` and returns the patched image.
/// Production builds would verify signatures and swap boot sectors atomically.
pub fn apply_delta_wave(base: &[u8], delta: &[u8]) -> Result<Vec<u8>, ()> {
    if delta.is_empty() {
        return Ok(base.to_vec());
    }

    let mut output = base.to_vec();
    for (index, patch_byte) in delta.iter().enumerate() {
        if index >= output.len() {
            output.push(*patch_byte);
        } else {
            output[index] ^= patch_byte;
        }
    }
    Ok(output)
}

/// Stores a patched blob in the Holographic File System and returns its resonance signature.
pub fn commit_patched_image(base: &[u8], delta: &[u8]) -> Result<u64, ()> {
    let patched = apply_delta_wave(base, delta)?;
    Ok(crate::hfs::manifest_data_global(&patched))
}
