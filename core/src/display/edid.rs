//! Sovereign EDID override — maximize reported refresh from raw DDC data.

/// Processed monitor capability profile after Utah-OS optimization.
#[derive(Clone, Copy, Debug)]
pub struct MonitorCapabilitiesProfile {
    pub manufacturer_id: [u8; 3],
    pub product_code: u16,
    pub absolute_max_pixel_clock_megahertz: u32,
    pub supports_high_dynamic_range_hdr: bool,
    pub optimized_refresh_rate_ceiling_hz: u32,
}

impl MonitorCapabilitiesProfile {
    /// Parses a 128-byte EDID block and applies performance-oriented refresh ceilings.
    pub fn parse_and_optimize_edid(raw_edid_byte_matrix: &[u8; 128]) -> Self {
        let raw_manufacturer_bits =
            ((raw_edid_byte_matrix[8] as u16) << 8) | (raw_edid_byte_matrix[9] as u16);
        let mut manufacturer_id = [b' '; 3];
        manufacturer_id[0] = (((raw_manufacturer_bits >> 10) & 0x1F) + 64) as u8;
        manufacturer_id[1] = (((raw_manufacturer_bits >> 5) & 0x1F) + 64) as u8;
        manufacturer_id[2] = ((raw_manufacturer_bits & 0x1F) + 64) as u8;

        let product_code =
            ((raw_edid_byte_matrix[11] as u16) << 8) | (raw_edid_byte_matrix[10] as u16);

        let base_pixel_clock_tens_of_khz =
            ((raw_edid_byte_matrix[55] as u32) << 8) | (raw_edid_byte_matrix[54] as u32);
        let realized_max_clock_mhz = (base_pixel_clock_tens_of_khz.saturating_mul(10)) / 1000;

        let digital_input_definition_byte = raw_edid_byte_matrix[20];
        let supports_high_dynamic_range_hdr = (digital_input_definition_byte & 0x80) != 0;

        let _verified_native_refresh_floor = raw_edid_byte_matrix[35];
        let optimized_refresh_rate_ceiling_hz = if realized_max_clock_mhz > 300 {
            240
        } else if realized_max_clock_mhz > 150 {
            144
        } else if realized_max_clock_mhz > 0 {
            75
        } else {
            60
        };

        MonitorCapabilitiesProfile {
            manufacturer_id,
            product_code,
            absolute_max_pixel_clock_megahertz: realized_max_clock_mhz,
            supports_high_dynamic_range_hdr,
            optimized_refresh_rate_ceiling_hz,
        }
    }

    /// Default profile when GOP/DDC probe is unavailable (QEMU / early boot).
    pub const fn simulated_primary() -> Self {
        MonitorCapabilitiesProfile {
            manufacturer_id: *b"SIM",
            product_code: 0,
            absolute_max_pixel_clock_megahertz: 240,
            supports_high_dynamic_range_hdr: false,
            optimized_refresh_rate_ceiling_hz: 144,
        }
    }

    pub const fn simulated_secondary() -> Self {
        MonitorCapabilitiesProfile {
            manufacturer_id: *b"AUX",
            product_code: 0,
            absolute_max_pixel_clock_megahertz: 140,
            supports_high_dynamic_range_hdr: false,
            optimized_refresh_rate_ceiling_hz: 75,
        }
    }
}
