// Cursor navigation and
// kickback module

use macroquad::prelude::*;

use crate::editor_audio::EditorAudio;

#[allow(dead_code)]
pub struct EditorCursor {
    pub xy: (usize, usize)
}

impl EditorCursor {
    #[allow(dead_code)]
    pub fn new() -> EditorCursor {
        EditorCursor { xy: (0, 0) }
    }
}

// pub const CURSOR_LINE_TO_WIDTH: bool = true;

#[allow(dead_code)]
/// Standard cursor navigation
pub fn file_text_navigation(cursor: &mut (usize, usize), text: &mut Vec<String>, audio: &EditorAudio) {
    if text.is_empty() {
        cursor.0 = 0;
        cursor.1 = 0;
        return;
    }

    // Clamp cursor to valid line
    cursor.1 = cursor.1.min(text.len() - 1);
    cursor.0 = cursor.0.min(text[cursor.1].len());

    if is_key_pressed(KeyCode::Up) {
        if cursor.1 > 0 {
            audio.play_nav();
            cursor.1 -= 1;
            cursor.0 = text[cursor.1].len().min(cursor.0);
        }
    }

    if is_key_pressed(KeyCode::Down) {
        if cursor.1 + 1 < text.len() {
            audio.play_nav();
            cursor.1 += 1;
            cursor.0 = text[cursor.1].len().min(cursor.0);
        }
    }

    if is_key_pressed(KeyCode::Left) {
        if cursor.0 > 0 {
            audio.play_nav();
            cursor.0 -= 1;
        } else if cursor.1 > 0 {
            audio.play_nav();
            cursor.1 -= 1;
            cursor.0 = text[cursor.1].len();
        }
    }

    if is_key_pressed(KeyCode::Right) {
        if cursor.0 < text[cursor.1].len() {
            audio.play_nav();
            cursor.0 += 1;
        } else if cursor.1 + 1 < text.len() {
            audio.play_nav();
            cursor.1 += 1;
            cursor.0 = 0;
        }
    }
}

#[allow(dead_code)]
/// Special navigation with LCTRL movement
pub fn file_text_special_navigation(cursor: &mut (usize, usize), text: &mut Vec<String>, audio: &EditorAudio) {
    if text.is_empty() {
        cursor.0 = 0;
        cursor.1 = 0;
        return;
    }

    let cursor_special_vertical_movement = 5;

    if is_key_down(KeyCode::LeftShift) {
        // Even faster vertical movement
        if is_key_down(KeyCode::Up) {
            if cursor.1 > 1 {
                audio.play_nav();
                cursor.1 -= 1;
            } else if cursor.1 <= 1 {
                audio.play_nav();
                cursor.0 = 0;
            }
        }
    
        if is_key_down(KeyCode::Down) {
            if cursor.1 + 1 < text.len() {
                audio.play_nav();
                cursor.1 += 1;
            } else {
                audio.play_nav();
                cursor.1 = text.len() - 1;
            }
        }   
    } else {
        // Faster verical movement
        if is_key_pressed(KeyCode::Up) {
            if cursor.1 > cursor_special_vertical_movement {
                audio.play_nav();
                cursor.1 -= cursor_special_vertical_movement;
            } else if cursor.1 <= 1 {
                audio.play_nav();
                cursor.0 = 0;
            }
        }
    
        if is_key_pressed(KeyCode::Down) {
            if cursor.1 + cursor_special_vertical_movement < text.len() {
                audio.play_nav();
                cursor.1 += cursor_special_vertical_movement;
            } else {
                audio.play_nav();
                cursor.1 = text.len() - 1;
            }
        }
    }

    // Clamp cursor to valid line
    cursor.1 = cursor.1.min(text.len() - 1);
    cursor.0 = cursor.0.min(text[cursor.1].len());

    let line_len = text[cursor.1].len();
    let left_steps_to_whitespace = calibrate_distance_to_whitespace_or_character(false, cursor.0, &text[cursor.1]);
    let right_steps_to_whitespace = calibrate_distance_to_whitespace_or_character(true, cursor.0, &text[cursor.1]);

    if is_key_pressed(KeyCode::Left) {
        if cursor.0 > 0 {
            audio.play_nav();
            cursor.0 = cursor.0.saturating_sub(left_steps_to_whitespace);
        } else if cursor.1 > 0 {
            audio.play_nav();
            cursor.1 -= 1;
            cursor.0 = text[cursor.1].len();
        }
    }

    if is_key_pressed(KeyCode::Right) {
        if cursor.0 < line_len {
            audio.play_nav();
            cursor.0 += right_steps_to_whitespace.min(line_len - cursor.0);
        } else if cursor.1 + 1 < text.len() {
            audio.play_nav();
            cursor.1 += 1;
            cursor.0 = 0;
        }
    }
}

/// Calculate the distance from the left or right of a whitespace if the cursor is inside text
/// or a character if the cursor is inside whitespace
#[allow(dead_code)]
fn calibrate_distance_to_whitespace_or_character(leftorright: bool, cursor_idx: usize, line: &str) -> usize {
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
