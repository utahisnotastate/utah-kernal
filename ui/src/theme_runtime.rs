//! Global active theme state.

use crate::theme::{SystemTheme, ThemePreset};
use spin::Mutex;

static ACTIVE_THEME: Mutex<SystemTheme> = Mutex::new(SystemTheme::obtain_preset(ThemePreset::Dark));

pub fn set_preset(preset: ThemePreset) {
    *ACTIVE_THEME.lock() = SystemTheme::obtain_preset(preset);
}

pub fn apply_vibe_modification(intent: &str) {
    ACTIVE_THEME.lock().execute_vibe_modification(intent);
}

pub fn active_theme() -> SystemTheme {
    *ACTIVE_THEME.lock()
}

pub fn render_desktop() {
    let theme = active_theme();
    crate::manifold::render_total_os_manifold(&theme);
}
