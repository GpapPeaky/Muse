
use macroquad::prelude::*;
use miniquad::{conf::Icon};

mod editor_camera;
use editor_camera::*;

mod editor_console;
use editor_console::*;
use crate::editor_console::editor_file::*;

mod editor_audio;
use editor_audio::*;

mod editor_cursor;
use editor_cursor::*;

mod editor_text;
use editor_text::*;

mod editor_pallete;
use editor_pallete::*;

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
    show_mouse(false);
    
    // Editor camera
    let mut ec = EditorCamera::new();
    // File system
    let mut efs = EditorFileSystem::new();
    // Editor audio
    let audio = EditorAudio::new().await;
    // Editor general text stylizer
    let mut gts = EditorGeneralTextStylizer::new().await;
    // Editor Cursor
    let mut file_cursor = EditorCursor::new();
    // Console
    let mut console = EditorConsole::new();
    // Actual file text
    let mut file_text = vec![];

    let insert_word_w = measure_text("INSERT MODE", None, MODE_FONT_SIZE as u16, 1.0).width;
    let console_word_w = measure_text("CONSOLE MODE", None, MODE_FONT_SIZE as u16, 1.0).width;
    
    loop {
        clear_background(BACKGROUND_COLOR);

        draw(&mut file_text, file_cursor.xy.0, file_cursor.xy.1, &mut gts, &console, &mut ec);
        if console.mode {
            draw_dir_contents(&efs.current_file, &efs.current_dir, console.directive.to_string());
        }

        if !console.mode {
            record_keyboard_to_file_text(&mut file_cursor, &mut file_text, &audio, &mut console,  &mut gts, &mut efs);

            let mut fname = path_buffer_file_to_string(&efs.current_file);
            if efs.unsaved_changes {
                fname = format!("*{}", path_buffer_file_to_string(&efs.current_file));
            }

            draw_text("INSERT MODE", 15.0, MODE_FONT_SIZE + MODE_Y_MARGIN - 15.0, MODE_FONT_SIZE, COMPOSITE_TYPE_COLOR);
            draw_text(&path_buffer_to_string(&efs.current_dir), insert_word_w + 25.0, MODE_FONT_SIZE + MODE_Y_MARGIN - 15.0, MODE_FONT_SIZE, FOLDER_COLOR);
            draw_text(&fname, insert_word_w + 25.0, MODE_FONT_SIZE + MODE_Y_MARGIN + 15.0, MODE_FONT_SIZE, FILE_COLOR);
        } else {
            console.record_keyboard_to_console_text(&audio, &mut efs, &mut file_text);
            
            let mut fname = path_buffer_file_to_string(&efs.current_file);
            if efs.unsaved_changes {
                fname = format!("*{}", path_buffer_file_to_string(&efs.current_file));
            }
            
            draw_text("CONSOLE MODE", 15.0, MODE_FONT_SIZE + MODE_Y_MARGIN - 15.0, MODE_FONT_SIZE, COMPOSITE_TYPE_COLOR,);
            draw_text(&path_buffer_to_string(&efs.current_dir), console_word_w + 25.0, MODE_FONT_SIZE + MODE_Y_MARGIN - 15.0, MODE_FONT_SIZE, FOLDER_COLOR);
            draw_text(&fname, console_word_w + 25.0, MODE_FONT_SIZE + MODE_Y_MARGIN + 15.0, MODE_FONT_SIZE, FILE_COLOR);
        }

        // Show message
        if console.showing_message {
            console_message(&console.message, console.showing_manual);

            if is_key_pressed(KeyCode::Escape) {
                console.showing_message = false;
                console.showing_manual = false;
                console.message.clear();
            }
        }
        
        next_frame().await;
    }
}
