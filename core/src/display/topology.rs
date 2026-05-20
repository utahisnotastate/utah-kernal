//! Unified Virtual Coordinate Topology — all monitors stitched into one desktop canvas.

extern crate alloc;

use alloc::vec::Vec;

/// Boundaries and VRAM layout of one physical display head.
#[derive(Clone, Copy, Debug)]
pub struct PhysicalMonitorHead {
    pub monitor_hardware_index: u32,
    pub video_memory_base_address: *mut u32,
    pub resolution_width_pixels: u32,
    pub resolution_height_pixels: u32,
    pub pixels_per_scan_line: u32,
    pub global_coordinate_offset_x: i32,
    pub global_coordinate_offset_y: i32,
    pub hardware_refresh_rate_hz: u32,
}

/// Stitches every connected head into a single virtual canvas.
pub struct GlobalDisplayTopology {
    pub attached_monitor_heads: Vec<PhysicalMonitorHead>,
    /// Owned VRAM backing stores (keeps pointers valid).
    frame_buffers: Vec<Vec<u32>>,
    pub combined_virtual_width: u32,
    pub combined_virtual_height: u32,
}

impl GlobalDisplayTopology {
    pub fn new() -> Self {
        GlobalDisplayTopology {
            attached_monitor_heads: Vec::new(),
            frame_buffers: Vec::new(),
            combined_virtual_width: 0,
            combined_virtual_height: 0,
        }
    }

    /// Registers a monitor laid out left-to-right on the unified canvas.
    pub fn register_monitor_head(
        &mut self,
        width: u32,
        height: u32,
        scan_line_stride: u32,
        refresh_hz: u32,
    ) -> u32 {
        let pixel_count = (width as usize).saturating_mul(height as usize);
        let mut buffer = Vec::new();
        buffer.resize(pixel_count, 0);
        let vram_pointer = buffer.as_mut_ptr();
        self.frame_buffers.push(buffer);

        let unique_index = self.attached_monitor_heads.len() as u32;
        let offset_x = self.combined_virtual_width as i32;
        let offset_y = 0;

        let head = PhysicalMonitorHead {
            monitor_hardware_index: unique_index,
            video_memory_base_address: vram_pointer,
            resolution_width_pixels: width,
            resolution_height_pixels: height,
            pixels_per_scan_line: scan_line_stride,
            global_coordinate_offset_x: offset_x,
            global_coordinate_offset_y: offset_y,
            hardware_refresh_rate_hz: refresh_hz,
        };

        self.attached_monitor_heads.push(head);
        self.combined_virtual_width += width;
        if height > self.combined_virtual_height {
            self.combined_virtual_height = height;
        }
        unique_index
    }

    /// Maps a head's external VRAM pointer (e.g. GOP framebuffer) to an existing buffer slot.
    pub fn register_monitor_head_external(
        &mut self,
        vram_pointer: *mut u32,
        width: u32,
        height: u32,
        scan_line_stride: u32,
        refresh_hz: u32,
    ) -> u32 {
        let unique_index = self.attached_monitor_heads.len() as u32;
        let offset_x = self.combined_virtual_width as i32;
        let head = PhysicalMonitorHead {
            monitor_hardware_index: unique_index,
            video_memory_base_address: vram_pointer,
            resolution_width_pixels: width,
            resolution_height_pixels: height,
            pixels_per_scan_line: scan_line_stride,
            global_coordinate_offset_x: offset_x,
            global_coordinate_offset_y: 0,
            hardware_refresh_rate_hz: refresh_hz,
        };
        self.attached_monitor_heads.push(head);
        self.combined_virtual_width += width;
        if height > self.combined_virtual_height {
            self.combined_virtual_height = height;
        }
        unique_index
    }
}

// Kernel-owned VRAM pointers are stable after init; safe behind Mutex.
unsafe impl Send for GlobalDisplayTopology {}
unsafe impl Sync for GlobalDisplayTopology {}

impl GlobalDisplayTopology {
    pub fn resolve_physical_coordinates(
        &self,
        global_x: i32,
        global_y: i32,
    ) -> Option<(PhysicalMonitorHead, u32, u32)> {
        for monitor in &self.attached_monitor_heads {
            let left = monitor.global_coordinate_offset_x;
            let right = monitor.global_coordinate_offset_x + monitor.resolution_width_pixels as i32;
            let top = monitor.global_coordinate_offset_y;
            let bottom = monitor.global_coordinate_offset_y + monitor.resolution_height_pixels as i32;

            if global_x >= left && global_x < right && global_y >= top && global_y < bottom {
                let local_x = (global_x - left) as u32;
                let local_y = (global_y - top) as u32;
                return Some((*monitor, local_x, local_y));
            }
        }
        None
    }

    pub fn head_count(&self) -> usize {
        self.attached_monitor_heads.len()
    }

    pub fn highest_refresh_head(&self) -> Option<PhysicalMonitorHead> {
        self.attached_monitor_heads
            .iter()
            .max_by_key(|m| m.hardware_refresh_rate_hz)
            .copied()
    }
}
