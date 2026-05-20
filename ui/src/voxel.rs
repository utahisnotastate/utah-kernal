//! Dynamic voxel clouds — UI elements as fluid particle fields (mouse / gaze vectors).

use crate::framebuffer::{Framebuffer, FRAMEBUFFER_HEIGHT, FRAMEBUFFER_WIDTH};
use crate::glass::render_interface_node;

/// Renders a refracting voxel cloud steered by an interaction vector (mouse or gaze).
pub fn draw_dynamic_voxel_cloud(
    origin_x: i32,
    origin_y: i32,
    vector_x: i32,
    vector_y: i32,
    intensity: u8,
) {
    let mut target = Framebuffer::new();
    let particle_count = 24i32;
    let spread = intensity as i32 / 8 + 4;

    for index in 0..particle_count {
        let phase = index.wrapping_mul(17);
        let offset_x = (vector_x * index / particle_count) + ((phase % spread) - spread / 2);
        let offset_y = (vector_y * index / particle_count) + (((phase / 3) % spread) - spread / 2);
        let px = origin_x + offset_x;
        let py = origin_y + offset_y;

        if px >= 0
            && py >= 0
            && (px as usize) < FRAMEBUFFER_WIDTH
            && (py as usize) < FRAMEBUFFER_HEIGHT
        {
            let fade = intensity.saturating_sub((index as u8).saturating_mul(3));
            render_interface_node(px, py, fade);

            // Soft halo pixel
            let halo = fade / 2;
            target.write_pixel(px + 1, py, 0x40, 0x80, 0xFF, halo);
            target.write_pixel(px, py + 1, 0x30, 0x60, 0xC0, halo);
        }
    }
}
