//! Linear framebuffer — direct VRAM-style memory for Glass-Forge.

use core::ptr;

/// Default holographic canvas width (matches master-copy stride).
pub const FRAMEBUFFER_WIDTH: usize = 800;
/// Default holographic canvas height.
pub const FRAMEBUFFER_HEIGHT: usize = 600;
/// Bytes per pixel (BGRA).
pub const BYTES_PER_PIXEL: usize = 4;

static mut FRAMEBUFFER_STORAGE: [u8; FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT * BYTES_PER_PIXEL] =
    [0; FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT * BYTES_PER_PIXEL];

static mut PHYSICAL_FRAMEBUFFER_BASE: *mut u8 = ptr::null_mut();

#[derive(Clone, Copy, Debug)]
pub struct FramebufferConfig {
    pub width: usize,
    pub height: usize,
    pub stride: usize,
}

impl Default for FramebufferConfig {
    fn default() -> Self {
        Self {
            width: FRAMEBUFFER_WIDTH,
            height: FRAMEBUFFER_HEIGHT,
            stride: FRAMEBUFFER_WIDTH,
        }
    }
}

pub struct Framebuffer {
    config: FramebufferConfig,
}

impl Framebuffer {
    pub fn new() -> Self {
        Self {
            config: FramebufferConfig::default(),
        }
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8, a: u8) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.config.width || y >= self.config.height {
            return;
        }
        let offset = (y * self.config.stride + x) * BYTES_PER_PIXEL;
        unsafe {
            let buffer = &mut FRAMEBUFFER_STORAGE;
            if offset + 3 >= buffer.len() {
                return;
            }
            buffer[offset] = b;
            buffer[offset + 1] = g;
            buffer[offset + 2] = r;
            buffer[offset + 3] = a;

            if !PHYSICAL_FRAMEBUFFER_BASE.is_null() {
                ptr::write(PHYSICAL_FRAMEBUFFER_BASE.add(offset), b);
                ptr::write(PHYSICAL_FRAMEBUFFER_BASE.add(offset + 1), g);
                ptr::write(PHYSICAL_FRAMEBUFFER_BASE.add(offset + 2), r);
                ptr::write(PHYSICAL_FRAMEBUFFER_BASE.add(offset + 3), a);
            }
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32, r: u8, g: u8, b: u8, a: u8) {
        for row in 0..h {
            for col in 0..w {
                self.write_pixel(x + col as i32, y + row as i32, r, g, b, a);
            }
        }
    }
}

pub fn map_physical_framebuffer(base: *mut u8) {
    unsafe {
        PHYSICAL_FRAMEBUFFER_BASE = base;
    }
}

pub fn clear_framebuffer(r: u8, g: u8, b: u8) {
    let mut fb = Framebuffer::new();
    fb.fill_rect(
        0,
        0,
        FRAMEBUFFER_WIDTH as u32,
        FRAMEBUFFER_HEIGHT as u32,
        r,
        g,
        b,
        255,
    );
}
