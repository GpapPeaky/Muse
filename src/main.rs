use macroquad::prelude::*;

mod editor_console;
use editor_console::*;

mod editor_audio;
use editor_audio::*;

mod editor_cursor;
use editor_cursor::*;

mod editor_text;
use editor_text::*;

#[macroquad::main("Muse")]
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
        } else {
            console.record_keyboard_to_console_text(&audio);
        }

        draw(&mut file_text, file_cursor.xy.0, file_cursor.xy.1, &mut gts, &console);

        draw_fps();

        next_frame().await;
    }
}
