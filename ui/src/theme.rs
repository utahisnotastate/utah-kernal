//! Theme Registry Matrix — preset palettes and runtime vibe-code string parser.

/// 24-bit RGB triplet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorRgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Styling specification for the Glass-Forge graphics pipeline.
#[derive(Clone, Copy, Debug)]
pub struct SystemTheme {
    pub background: ColorRgb,
    pub surface: ColorRgb,
    pub primary_accent: ColorRgb,
    pub secondary_accent: ColorRgb,
    pub interactive_element: ColorRgb,
    pub text_primary: ColorRgb,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThemePreset {
    Dark = 0,
    Golden = 1,
    Light = 2,
    Linda = 3,
    Occult = 4,
}

impl ThemePreset {
    pub fn from_u32(value: u32) -> Self {
        match value {
            1 => ThemePreset::Golden,
            2 => ThemePreset::Light,
            3 => ThemePreset::Linda,
            4 => ThemePreset::Occult,
            _ => ThemePreset::Dark,
        }
    }
}

impl SystemTheme {
    pub const fn obtain_preset(preset: ThemePreset) -> Self {
        match preset {
            ThemePreset::Dark => SystemTheme {
                background: ColorRgb { red: 15, green: 23, blue: 42 },
                surface: ColorRgb { red: 30, green: 41, blue: 59 },
                primary_accent: ColorRgb { red: 56, green: 189, blue: 248 },
                secondary_accent: ColorRgb { red: 99, green: 102, blue: 241 },
                interactive_element: ColorRgb { red: 51, green: 65, blue: 85 },
                text_primary: ColorRgb { red: 248, green: 250, blue: 252 },
            },
            ThemePreset::Golden => SystemTheme {
                background: ColorRgb { red: 20, green: 17, blue: 10 },
                surface: ColorRgb { red: 43, green: 34, blue: 16 },
                primary_accent: ColorRgb { red: 234, green: 179, blue: 8 },
                secondary_accent: ColorRgb { red: 202, green: 138, blue: 4 },
                interactive_element: ColorRgb { red: 67, green: 56, blue: 202 },
                text_primary: ColorRgb { red: 254, green: 249, blue: 195 },
            },
            ThemePreset::Light => SystemTheme {
                background: ColorRgb { red: 241, green: 245, blue: 249 },
                surface: ColorRgb { red: 255, green: 255, blue: 255 },
                primary_accent: ColorRgb { red: 37, green: 99, blue: 235 },
                secondary_accent: ColorRgb { red: 79, green: 70, blue: 229 },
                interactive_element: ColorRgb { red: 226, green: 232, blue: 240 },
                text_primary: ColorRgb { red: 15, green: 23, blue: 42 },
            },
            ThemePreset::Linda => SystemTheme {
                background: ColorRgb { red: 24, green: 14, blue: 36 },
                surface: ColorRgb { red: 45, green: 21, blue: 62 },
                primary_accent: ColorRgb { red: 244, green: 63, blue: 94 },
                secondary_accent: ColorRgb { red: 168, green: 85, blue: 247 },
                interactive_element: ColorRgb { red: 76, green: 29, blue: 149 },
                text_primary: ColorRgb { red: 253, green: 244, blue: 255 },
            },
            ThemePreset::Occult => SystemTheme {
                background: ColorRgb { red: 10, green: 4, blue: 4 },
                surface: ColorRgb { red: 28, green: 10, blue: 10 },
                primary_accent: ColorRgb { red: 220, green: 38, blue: 38 },
                secondary_accent: ColorRgb { red: 126, green: 34, blue: 206 },
                interactive_element: ColorRgb { red: 69, green: 10, blue: 10 },
                text_primary: ColorRgb { red: 239, green: 68, blue: 68 },
            },
        }
    }

    /// Parses intent strings like `primary_accent: 250,204,21; secondary_accent: 236,72,153;`
    pub fn execute_vibe_modification(&mut self, source_intent: &str) {
        for assigning_segment in source_intent.split(';') {
            let mut element_parts = assigning_segment.split(':');
            let Some(key) = element_parts.next() else {
                continue;
            };
            let Some(value) = element_parts.next() else {
                continue;
            };
            let stripped_key = key.trim();
            let stripped_value = value.trim();

            let mut channels = stripped_value.split(',');
            let Some(r) = channels.next().and_then(|v| parse_u8(v.trim())) else {
                continue;
            };
            let Some(g) = channels.next().and_then(|v| parse_u8(v.trim())) else {
                continue;
            };
            let Some(b) = channels.next().and_then(|v| parse_u8(v.trim())) else {
                continue;
            };
            let color = ColorRgb { red: r, green: g, blue: b };
            match stripped_key {
                "background" => self.background = color,
                "surface" => self.surface = color,
                "primary_accent" => self.primary_accent = color,
                "secondary_accent" => self.secondary_accent = color,
                "interactive_element" => self.interactive_element = color,
                "text_primary" => self.text_primary = color,
                _ => {}
            }
        }
    }
}

fn parse_u8(text: &str) -> Option<u8> {
    let mut value: u32 = 0;
    for byte in text.bytes() {
        if !byte.is_ascii_digit() {
            return None;
        }
        value = value * 10 + (byte - b'0') as u32;
        if value > 255 {
            return None;
        }
    }
    Some(value as u8)
}
