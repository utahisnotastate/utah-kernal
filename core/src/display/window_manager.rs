//! Intent-based application pinning across the unified virtual desktop.

extern crate alloc;

use alloc::vec::Vec;
use glass_forge::ColorRgb;

use super::topology::GlobalDisplayTopology;

/// Window placement rule.
#[derive(Clone, Copy, Debug)]
pub enum ApplicationPinRule {
    StrictMonitorIndex(u32),
    HighestPerformanceDisplay,
    SpanAllAvailableMonitors,
}

/// Application window frame in global coordinates.
#[derive(Clone, Debug)]
pub struct StructuralWindowFrame {
    pub application_title_id: &'static str,
    pub workspace_position_x: i32,
    pub workspace_position_y: i32,
    pub frame_width_pixels: u32,
    pub frame_height_pixels: u32,
    pub primary_placement_rule: ApplicationPinRule,
}

impl StructuralWindowFrame {
    pub const fn new(title: &'static str, width: u32, height: u32, rule: ApplicationPinRule) -> Self {
        StructuralWindowFrame {
            application_title_id: title,
            workspace_position_x: 0,
            workspace_position_y: 0,
            frame_width_pixels: width,
            frame_height_pixels: height,
            primary_placement_rule: rule,
        }
    }

    pub fn calculate_automated_placement(&mut self, system_topology: &GlobalDisplayTopology) {
        match self.primary_placement_rule {
            ApplicationPinRule::StrictMonitorIndex(target_index) => {
                if let Some(target_screen) = system_topology
                    .attached_monitor_heads
                    .iter()
                    .find(|m| m.monitor_hardware_index == target_index)
                {
                    self.workspace_position_x = target_screen.global_coordinate_offset_x;
                    self.workspace_position_y = target_screen.global_coordinate_offset_y;
                    self.frame_width_pixels = self
                        .frame_width_pixels
                        .min(target_screen.resolution_width_pixels.saturating_sub(8));
                    self.frame_height_pixels = self
                        .frame_height_pixels
                        .min(target_screen.resolution_height_pixels.saturating_sub(8));
                }
            }
            ApplicationPinRule::HighestPerformanceDisplay => {
                if let Some(fastest) = system_topology.highest_refresh_head() {
                    self.workspace_position_x = fastest.global_coordinate_offset_x + 16;
                    self.workspace_position_y = fastest.global_coordinate_offset_y + 16;
                }
            }
            ApplicationPinRule::SpanAllAvailableMonitors => {
                self.workspace_position_x = 0;
                self.workspace_position_y = 0;
                self.frame_width_pixels = system_topology.combined_virtual_width;
                self.frame_height_pixels = system_topology.combined_virtual_height;
            }
        }
    }

    /// Draws a 2px accent border into all intersecting monitor framebuffers.
    pub fn commit_window_draw_pulses(
        &self,
        system_topology: &GlobalDisplayTopology,
        boundary_color: ColorRgb,
    ) {
        let packed = pack_rgb(boundary_color);
        let w = self.frame_width_pixels;
        let h = self.frame_height_pixels;
        if w == 0 || h == 0 {
            return;
        }

        for local_x in 0..w {
            write_global_pixel(system_topology, self.workspace_position_x + local_x as i32, self.workspace_position_y, packed);
            write_global_pixel(
                system_topology,
                self.workspace_position_x + local_x as i32,
                self.workspace_position_y + h as i32 - 1,
                packed,
            );
        }
        for local_y in 0..h {
            write_global_pixel(system_topology, self.workspace_position_x, self.workspace_position_y + local_y as i32, packed);
            write_global_pixel(
                system_topology,
                self.workspace_position_x + w as i32 - 1,
                self.workspace_position_y + local_y as i32,
                packed,
            );
        }
    }
}

fn pack_rgb(color: ColorRgb) -> u32 {
    ((color.red as u32) << 16) | ((color.green as u32) << 8) | (color.blue as u32)
}

fn write_global_pixel(topology: &GlobalDisplayTopology, global_x: i32, global_y: i32, packed: u32) {
    if let Some((head, local_x, local_y)) = topology.resolve_physical_coordinates(global_x, global_y)
    {
        let stride = head.pixels_per_scan_line.max(head.resolution_width_pixels);
        let offset = (local_y * stride + local_x) as isize;
        unsafe {
            if !head.video_memory_base_address.is_null() {
                head.video_memory_base_address
                    .offset(offset)
                    .write_volatile(packed);
            }
        }
    }
}

/// Registry of pinned windows for the session.
pub struct WindowPinRegistry {
    pub frames: Vec<StructuralWindowFrame>,
}

impl WindowPinRegistry {
    pub fn new() -> Self {
        WindowPinRegistry {
            frames: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.frames.clear();
    }

    pub fn register(&mut self, frame: StructuralWindowFrame) {
        self.frames.push(frame);
    }

    pub fn place_all(&mut self, topology: &GlobalDisplayTopology) {
        for frame in &mut self.frames {
            frame.calculate_automated_placement(topology);
        }
    }

    pub fn commit_all(&self, topology: &GlobalDisplayTopology, color: ColorRgb) {
        for frame in &self.frames {
            frame.commit_window_draw_pulses(topology, color);
        }
    }
}
