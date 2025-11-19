// Cursor navigation and
// kickback module

use macroquad::prelude::*;

use crate::audio::editor_audio::*;

pub struct EditorCursor {
    pub xy: (usize, usize),
}

impl EditorCursor {
    pub fn new() -> EditorCursor {
        EditorCursor {
            xy: (0, 0),
         }
    }
}

/// Standard cursor navigation
pub fn file_text_navigation(
    cursor: &mut EditorCursor, 
    text: &mut Vec<String>, 
    audio: &EditorAudio
) {
    if text.is_empty() {
        cursor.xy.0 = 0;
        cursor.xy.1 = 0;
        return;
    }

    // Clamp cursor to valid line
    cursor.xy.1 = cursor.xy.1.min(text.len() - 1);
    cursor.xy.0 = cursor.xy.0.min(text[cursor.xy.1].len());

    if is_key_pressed(KeyCode::Up) {
        if cursor.xy.1 > 0 {
            audio.play_nav();
            cursor.xy.1 -= 1;
            cursor.xy.0 = text[cursor.xy.1].len().min(cursor.xy.0);
        }
    }

    if is_key_pressed(KeyCode::Down) {
        if cursor.xy.1 + 1 < text.len() {
            audio.play_nav();
            cursor.xy.1 += 1;
            cursor.xy.0 = text[cursor.xy.1].len().min(cursor.xy.0);
        }
    }

    if is_key_pressed(KeyCode::Left) {
        if cursor.xy.0 > 0 {
            audio.play_nav();
            cursor.xy.0 -= 1;
        } else if cursor.xy.1 > 0 {
            audio.play_nav();
            cursor.xy.1 -= 1;
            cursor.xy.0 = text[cursor.xy.1].len();
        }
    }

    if is_key_pressed(KeyCode::Right) {
        if cursor.xy.0 < text[cursor.xy.1].len() {
            audio.play_nav();
            cursor.xy.0 += 1;
        } else if cursor.xy.1 + 1 < text.len() {
            audio.play_nav();
            cursor.xy.1 += 1;
            cursor.xy.0 = 0;
        }
    }
}

/// Special navigation with LCTRL movement
pub fn file_text_special_navigation(
    cursor: &mut EditorCursor, 
    text: &mut Vec<String>, 
    audio: &EditorAudio
) {
    if text.is_empty() {
        cursor.xy.0 = 0;
        cursor.xy.1 = 0;
        return;
    }

    let cursor_special_vertical_movement = 4;

    if is_key_down(KeyCode::LeftShift) {
        // Even faster vertical movement
        if is_key_down(KeyCode::Up) {
            if cursor.xy.1 > 1 {
                audio.play_nav();
                cursor.xy.1 -= 1;
            } else if cursor.xy.1 <= 1 {
                audio.play_nav();
                cursor.xy.0 = 0;
            }
        }

        if is_key_down(KeyCode::Down) {
            if cursor.xy.1 + 1 < text.len() {
                audio.play_nav();
                cursor.xy.1 += 1;
            } else {
                audio.play_nav();
                cursor.xy.1 = text.len() - 1;
            }
        }   
    } else {
        // Faster verical movement
        if is_key_pressed(KeyCode::Up) {
            if cursor.xy.1 > cursor_special_vertical_movement {
                audio.play_nav();
                cursor.xy.1 -= cursor_special_vertical_movement;
            } else if cursor.xy.1 <= 1 {
                audio.play_nav();
                cursor.xy.0 = 0;
            }
        }
    
        if is_key_pressed(KeyCode::Down) {
            if cursor.xy.1 + cursor_special_vertical_movement < text.len() {
                audio.play_nav();
                cursor.xy.1 += cursor_special_vertical_movement;
            } else {
                audio.play_nav();
                cursor.xy.1 = text.len() - 1;
            }
        }
    }

    // Clamp cursor to valid line
    cursor.xy.1 = cursor.xy.1.min(text.len() - 1);
    cursor.xy.0 = cursor.xy.0.min(text[cursor.xy.1].len());

    let line_len = text[cursor.xy.1].len();
    let left_steps_to_whitespace = calibrate_distance_to_whitespace_or_character(false, cursor.xy.0, &text[cursor.xy.1]);
    let right_steps_to_whitespace = calibrate_distance_to_whitespace_or_character(true, cursor.xy.0, &text[cursor.xy.1]);

    if is_key_pressed(KeyCode::Left) {
        if cursor.xy.0 > 0 {
            audio.play_nav();
            cursor.xy.0 = cursor.xy.0.saturating_sub(left_steps_to_whitespace);
        } else if cursor.xy.1 > 0 {
            audio.play_nav();
            cursor.xy.1 -= 1;
            cursor.xy.0 = text[cursor.xy.1].len();
        }
    }

    if is_key_pressed(KeyCode::Right) {
        if cursor.xy.0 < line_len {
            audio.play_nav();
            cursor.xy.0 += right_steps_to_whitespace.min(line_len - cursor.xy.0);
        } else if cursor.xy.1 + 1 < text.len() {
            audio.play_nav();
            cursor.xy.1 += 1;
            cursor.xy.0 = 0;
        }
    }
}

/// Calculate the distance from the left or right of a whitespace if the cursor is inside text
/// or a character if the cursor is inside whitespace
pub fn calibrate_distance_to_whitespace_or_character(
    leftorright: bool, 
    cursor_idx: usize, 
    line: &str
) -> usize {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    if len == 0 {
        return 0;
    }

    let mut cursor = cursor_idx.min(len);
    let mut steps = 0;

    // True right, false left
    if leftorright {
        if cursor >= len {
            return 0;
        }

        let is_not_special = !chars[cursor].is_alphanumeric();
        for i in cursor..len {
            if !chars[i].is_alphanumeric() && !is_not_special {
                break;
            }
            if chars[i].is_alphanumeric() && is_not_special {
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
        let is_not_special = !chars[cursor].is_alphanumeric();

        while cursor > 0 {
            if !chars[cursor - 1].is_alphanumeric() && !is_not_special {
                break;
            }
            if chars[cursor - 1].is_alphanumeric() && is_not_special {
                break;
            }
            cursor -= 1;
            steps += 1;
        }

        steps + 1
    }
}
