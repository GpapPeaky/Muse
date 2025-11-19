// Cursor navigation and
// kickback module

use macroquad::prelude::*;

use crate::audio::editor_audio::*;

pub struct EditorConsoleCursor {
    pub x: usize
}

impl EditorConsoleCursor {
    pub fn new() -> EditorConsoleCursor {
        EditorConsoleCursor { x: 0 }
    }
}

/// Standard console cursor navigation
pub fn console_text_navigation(
    cursor_x: &mut usize, 
    directive: &mut String, 
    audio: &EditorAudio
) {
    let cursor_x_pos = *cursor_x as i32;

    if is_key_pressed(KeyCode::Left) {
        if cursor_x_pos > 0 {
            audio.play_nav();
            *cursor_x -= 1;
        }
    }

    if is_key_pressed(KeyCode::Right) {
        if cursor_x_pos < directive.chars().count() as i32 {
            audio.play_nav();
            *cursor_x += 1;
        }
    }
}

