use miniquad::{conf::Icon};
use macroquad::prelude::*;

/// Window configuration
pub fn window_conf() -> Conf {
    let icon = Icon {
        small: *include_bytes!("../../assets/icon/muse16.bin"),   // 16x16 RGBA
        medium: *include_bytes!("../../assets/icon/muse32.bin"),  // 32x32 RGBA
        big: *include_bytes!("../../assets/icon/muse64.bin"),     // 64x64 RGBA
    };

    Conf {
        window_title: "Muse-V1.5.1".to_string(),
        icon: Some(icon),
        window_width: 1700,
        window_height: 1000,
        window_resizable: true,
        ..Default::default()
    }
}