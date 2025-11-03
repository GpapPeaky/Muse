// Cursor navigation and
// kickback module

use macroquad::prelude::*;

use crate::editor_audio::EditorAudio;

#[allow(dead_code)]
pub struct EditorConsoleCursor {
    pub x: usize
}

impl EditorConsoleCursor {
    #[allow(dead_code)]
    pub fn new() -> EditorConsoleCursor {
        EditorConsoleCursor { x: 0 }
    }
}

pub static CURSOR_LINE_TO_WIDTH: bool = true;

/// Standard console cursor navigation
#[allow(dead_code)] // Compiler won't shut the fuck up
pub fn console_text_navigation(cursor_x: &mut usize, directive: &mut String, audio: &EditorAudio) {
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

/// Calculate the distance from the left or right of a whitespace if the cursor is inside text
/// or a character if the cursor is inside whitespace
#[allow(dead_code)]
fn console_calibrate_distance_to_whitespace_or_character(leftorright: bool, cursor_idx: usize, line: &str) -> usize {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    if len == 0 {
        return 0;
    }

    let mut cursor = cursor_idx.min(len);
    let mut steps = 0;

    if leftorright {
        if cursor >= len {
            return 0;
        }

        let is_space = chars[cursor] == ' ';
        for i in cursor..len {
            if chars[i] == ' ' && !is_space {
                break;
            }
            if chars[i] != ' ' && is_space {
                break;
            }
            steps += 1;
        }

        return steps;
    } else {
        if cursor == 0 {
            return 0;
        }

        cursor -= 1;
        let is_space = chars[cursor] == ' ';

        while cursor > 0 {
            if chars[cursor - 1] == ' ' && !is_space {
                break;
            }
            if chars[cursor - 1] != ' ' && is_space {
                break;
            }
            cursor -= 1;
            steps += 1;
        }

        steps + 1
    }
}

/// Faster cursor navigation inside the file
/// only usable when the LCTRL key is down
#[allow(dead_code)]
pub fn console_text_special_navigation(cursor_x: &mut usize, text: &mut String, audio: &EditorAudio) {
    let left_steps_to_whitespace = console_calibrate_distance_to_whitespace_or_character(false, *cursor_x, &text);
    let right_steps_to_whitespace = console_calibrate_distance_to_whitespace_or_character(true, *cursor_x, &text);

    // Unsure what to do with this
    // if is_key_pressed(KeyCode::Up) {
    //     if cursor.1 > 0 {
    //         audio.play_nav();
    //         cursor.1 -= 1;
    //         cursor.0 = text[cursor.1].len();
    //     }
    // }

    // if is_key_pressed(KeyCode::Down) {
    //     if text.len() > cursor.1 + 1 {
    //         audio.play_nav();
    //         cursor.1 += 1;
    //         cursor.0 = text[cursor.1].len();
    //     }
    // }

    if is_key_pressed(KeyCode::Left) {
        if *cursor_x > 0 {
            audio.play_nav();
            *cursor_x = (*cursor_x).saturating_sub(left_steps_to_whitespace);
        }
    }
    
    if is_key_pressed(KeyCode::Right) {
        if *cursor_x < text.chars().count() {
            audio.play_nav();
            *cursor_x += right_steps_to_whitespace.min(text.chars().count() - *cursor_x);
        }
    }
}
