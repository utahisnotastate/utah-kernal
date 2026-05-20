//! Glass-morphic primitives — transparency dithering and particle-style nodes.

use crate::framebuffer::{Framebuffer, FRAMEBUFFER_HEIGHT, FRAMEBUFFER_WIDTH};

/// Master-copy primitive: direct framebuffer injection with glass dithering.
pub fn render_interface_node(node_x: i32, node_y: i32, intensity: u8) {
    let mut target = Framebuffer::new();
    let (r, g, b) = glass_rgb(intensity);

    for dy in 0..4 {
        for dx in 0..4 {
            let px = node_x + dx;
            let py = node_y + dy;
            if px >= 0
                && py >= 0
                && (px as usize) < FRAMEBUFFER_WIDTH
                && (py as usize) < FRAMEBUFFER_HEIGHT
            {
                let boost = if ((dx + dy) % 2) == 0 { 24 } else { 0 };
                let alpha = intensity.saturating_div(2).saturating_add(40);
                target.write_pixel(
                    px,
                    py,
                    r.saturating_add(boost),
                    g.saturating_add(boost),
                    b.saturating_add(boost / 2),
                    alpha,
                );
            }
        }
    }
}

/// Particle-system node (voxel cloud seed for buttons / windows).
pub fn draw_particle_node(center_x: i32, center_y: i32, radius: i32, intensity: u8) {
    let (r, g, b) = glass_rgb(intensity);
    let mut target = Framebuffer::new();
    let r_sq = radius * radius;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= r_sq {
                target.write_pixel(center_x + dx, center_y + dy, r, g, b, 220);
            }
        }
    }
}

/// Glass-morphic panel with soft border.
pub fn draw_glass_panel(x: i32, y: i32, width: u32, height: u32, intensity: u8) {
    let (r, g, b) = glass_rgb(intensity);
    let mut target = Framebuffer::new();
    target.fill_rect(x + 1, y + 1, width.saturating_sub(2), height.saturating_sub(2), r, g, b, 180);
    // Border
    target.fill_rect(x, y, width, 2, 0x50, 0x90, 0xFF, 255);
    target.fill_rect(x, y + height as i32 - 2, width, 2, 0x50, 0x90, 0xFF, 255);
    target.fill_rect(x, y, 2, height, 0x50, 0x90, 0xFF, 255);
    target.fill_rect(x + width as i32 - 2, y, 2, height, 0x50, 0x90, 0xFF, 255);
}

/// Utah-OS boot splash.
pub fn draw_boot_splash() {
    draw_glass_panel(40, 40, 720, 80, 180);
    draw_glass_panel(40, 140, 340, 400, 140);
    draw_glass_panel(420, 140, 340, 400, 160);

    draw_particle_node(120, 480, 28, 220);
    draw_particle_node(400, 480, 28, 200);
    draw_particle_node(680, 480, 28, 240);

    for index in 0..12 {
        let x = 80 + (index * 55);
        let y = 200 + ((index % 3) * 18);
        render_interface_node(x, y, (140 + index * 8) as u8);
    }

    draw_label(56, 64, b"Utah-OS Glass-Forge");
}

fn draw_label(x: i32, y: i32, text: &[u8]) {
    let mut target = Framebuffer::new();
    let mut cursor_x = x;
    for &ch in text {
        if ch == b' ' {
            cursor_x += 8;
            continue;
        }
        target.fill_rect(cursor_x, y, 6, 10, 0xE0, 0xF0, 0xFF, 255);
        cursor_x += 7;
    }
}

fn glass_rgb(intensity: u8) -> (u8, u8, u8) {
    let i = intensity as u32;
    (
        (0x30 + (i / 4)).min(255) as u8,
        (0x50 + (i / 3)).min(255) as u8,
        (0x90 + (i / 2)).min(255) as u8,
    )
}
