use macroquad::prelude::*;
use miniquad::{conf::Icon};

mod editor_console;
use editor_console::*;

mod editor_audio;
use editor_audio::*;

mod editor_cursor;
use editor_cursor::*;

mod editor_text;
use editor_text::*;

/// Window configuration
fn window_conf() -> Conf {
    let icon = Icon {
        small: *include_bytes!("../assets/icon/muse16.bin"),   // 16x16 RGBA
        medium: *include_bytes!("../assets/icon/muse32.bin"),  // 32x32 RGBA
        big: *include_bytes!("../assets/icon/muse64.bin"),     // 64x64 RGBA
    };

    Conf {
        window_title: "Muse".to_string(),
        icon: Some(icon),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_fullscreen(true);
    
    // Editor audio
    let audio = EditorAudio::new().await;
    // Editor general text stylizer
    let mut gts = EditorGeneralTextStylizer::new().await;
    // Editor Cursor
    let mut file_cursor = EditorCursor::new(); // Cursor's x and y
    // Console
    let mut console = EditorConsole::new();

    let mut file_text = vec![];
    
    loop {
        clear_background(BACKGROUND_COLOR);

        if !console.mode {
            record_keyboard_to_file_text(&mut file_cursor, &mut file_text, &audio, &mut console,  &mut gts);
            draw_text("INSERT MODE", 15.0, MODE_FONT_SIZE + MODE_Y_MARGIN - 15.0, MODE_FONT_SIZE, COMPOSITE_TYPE_COLOR);
        } else {
            console.record_keyboard_to_console_text(&audio);
            draw_text("CONSOLE MODE", 15.0, MODE_FONT_SIZE + MODE_Y_MARGIN - 15.0, MODE_FONT_SIZE, COMPOSITE_TYPE_COLOR);
        }

        draw(&mut file_text, file_cursor.xy.0, file_cursor.xy.1, &mut gts, &console);

        next_frame().await;
    }
}
