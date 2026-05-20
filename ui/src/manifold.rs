//! Utah-OS desktop manifold — taskbar, app panels, browser workspace (theme-driven).

use crate::framebuffer::Framebuffer;
use crate::theme::{ColorRgb, SystemTheme};

/// Renders the full Utah-OS visual workspace using the active theme.
pub fn render_total_os_manifold(active_style: &SystemTheme) {
    let mut fb = Framebuffer::new();
    let w = fb.config.width as u32;
    let h = fb.config.height as u32;

    fb.fill_rect(0, 0, w, h, active_style.background.red, active_style.background.green, active_style.background.blue, 255);

    let taskbar_h = 45u32.min(h);
    fb.fill_rect(0, 0, w, taskbar_h, active_style.surface.red, active_style.surface.green, active_style.surface.blue, 255);
    for x in 0..w {
        fb.write_pixel(x as i32, (taskbar_h.saturating_sub(1)) as i32, active_style.primary_accent.red, active_style.primary_accent.green, active_style.primary_accent.blue, 255);
    }

    let panel_top = 70i32;
    let calc_w = 260u32;
    let calc_h = 400u32;
    let explorer_x = 300i32;
    let explorer_w = 460u32;
    let browser_y = 490i32;
    let browser_w = 740u32.min(w.saturating_sub(40));
    let browser_h = 270u32;

    render_glass_panel_themed(&mut fb, 20, panel_top, calc_w, calc_h, active_style);
    render_glass_panel_themed(&mut fb, explorer_x, panel_top, explorer_w, calc_h, active_style);
    render_glass_panel_themed(&mut fb, 20, browser_y, browser_w, browser_h, active_style);

    for x in 22..278i32 {
        fb.write_pixel(x, panel_top + 1, active_style.secondary_accent.red, active_style.secondary_accent.green, active_style.secondary_accent.blue, 255);
        fb.write_pixel(x, panel_top + 2, active_style.secondary_accent.red, active_style.secondary_accent.green, active_style.secondary_accent.blue, 255);
    }

    draw_label_strip(&mut fb, 28, 12, b"Utah-OS", active_style.text_primary);
    draw_label_strip(&mut fb, 120, 12, b"Calculator", active_style.text_primary);
    draw_label_strip(&mut fb, 320, 12, b"HFS Explorer", active_style.text_primary);
}

fn render_glass_panel_themed(fb: &mut Framebuffer, pos_x: i32, pos_y: i32, width: u32, height: u32, style: &SystemTheme) {
    fb.fill_rect(pos_x, pos_y, width, height, style.surface.red, style.surface.green, style.surface.blue, 220);
    let x0 = pos_x;
    let y0 = pos_y;
    let x1 = pos_x + width as i32 - 1;
    let y1 = pos_y + height as i32 - 1;
    let border = style.interactive_element;
    for x in x0..=x1 {
        fb.write_pixel(x, y0, border.red, border.green, border.blue, 255);
        fb.write_pixel(x, y1, border.red, border.green, border.blue, 255);
    }
    for y in y0..=y1 {
        fb.write_pixel(x0, y, border.red, border.green, border.blue, 255);
        fb.write_pixel(x1, y, border.red, border.green, border.blue, 255);
    }
}

fn draw_label_strip(fb: &mut Framebuffer, x: i32, y: i32, text: &[u8], color: ColorRgb) {
    let mut cursor = x;
    for &ch in text {
        if ch == b' ' {
            cursor += 8;
            continue;
        }
        fb.fill_rect(cursor, y, 6, 10, color.red, color.green, color.blue, 255);
        cursor += 7;
    }
}
