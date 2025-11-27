// Cursor navigation and
// kickback module

use std::collections::{HashMap};

use macroquad::prelude::*;

use crate::audio::editor_audio::*;

pub const CURSOR_WORD_OFFSET: f32 = 600.0;

pub const CURSOR_CONTINIOUS_PRESS_INITAL_DELAY: f64 = 0.006;

pub const CURSOR_CONTINIOUS_PRESS_DELAY: f64 = 0.095;

pub struct EditorCursor {
    pub xy: (usize, usize),
    pub word: String,
    pub key_timers: HashMap<(KeyCode, Option<KeyCode>), f64>,
}

impl EditorCursor {
    pub fn new() -> EditorCursor {
        EditorCursor {
            xy: (0, 0),
            word: String::from(""),
            key_timers: HashMap::new()
        }
    }

    /// Returns true if key is pressed with continuous repeat
    pub fn is_combo_active(&mut self, key: KeyCode, modifier: Option<KeyCode>, dt: f64) -> bool {
        let repeat_delay = CURSOR_CONTINIOUS_PRESS_INITAL_DELAY;
        let repeat_rate = CURSOR_CONTINIOUS_PRESS_DELAY;

        if is_key_down(key) && modifier.map_or(true, |m| is_key_down(m)) {
            let timer = self.key_timers.entry((key, modifier)).or_insert(repeat_delay);
            if *timer <= 0.0 {
                *timer = repeat_rate;
                return true;
            } else {
                *timer -= dt;
                return false;
            }
        } else {
            self.key_timers.remove(&(key, modifier));
            return false;
        }
    }
}

/// Find the cursor's word fragment
pub fn recognize_cursor_word(
    cursor: &mut EditorCursor,
    line: &String
) {
    // Find the character collection of the word, left and right
    // from the word_idx

    let cursor_idx = cursor.xy.0;
    let left_distance = calibrate_distance_to_whitespace(false, cursor_idx, line);
    let right_distance = calibrate_distance_to_whitespace(true, cursor_idx, line);
    
    // Index of where the word starts
    let left_cursor_idx = cursor_idx - left_distance;
    let right_cursor_idx = right_distance + cursor_idx;
    
    cursor.word = line[left_cursor_idx..right_cursor_idx].to_string();
}

/// Standard cursor navigation (with repeat timer)
pub fn file_text_navigation(
    cursor: &mut EditorCursor,
    text: &mut Vec<String>,
    audio: &EditorAudio,
    dt: f64,
) {
    if text.is_empty() {
        cursor.xy = (0, 0);
        return;
    }

    cursor.xy.1 = cursor.xy.1.min(text.len() - 1);
    cursor.xy.0 = cursor.xy.0.min(text[cursor.xy.1].len());

    // Up
    if cursor.is_combo_active(KeyCode::Up, None, dt) && cursor.xy.1 > 0 {
        cursor.xy.1 -= 1;
        cursor.xy.0 = cursor.xy.0.min(text[cursor.xy.1].len());
        audio.play_nav();
    }

    // Down
    if cursor.is_combo_active(KeyCode::Down, None, dt) && cursor.xy.1 + 1 < text.len() {
        cursor.xy.1 += 1;
        cursor.xy.0 = cursor.xy.0.min(text[cursor.xy.1].len());
        audio.play_nav();
    }

    // Left
    if cursor.is_combo_active(KeyCode::Left, None, dt) {
        if cursor.xy.0 > 0 {
            cursor.xy.0 -= 1;
        } else if cursor.xy.1 > 0 {
            cursor.xy.1 -= 1;
            cursor.xy.0 = text[cursor.xy.1].len();
        }
        audio.play_nav();
    }

    // Right
    if cursor.is_combo_active(KeyCode::Right, None, dt) {
        if cursor.xy.0 < text[cursor.xy.1].len() {
            cursor.xy.0 += 1;
        } else if cursor.xy.1 + 1 < text.len() {
            cursor.xy.1 += 1;
            cursor.xy.0 = 0;
        }
        audio.play_nav();
    }

    recognize_cursor_word(cursor, &text[cursor.xy.1]);
}

/// Special navigation with LCTRL movement
pub fn file_text_special_navigation(
    cursor: &mut EditorCursor, 
    text: &mut Vec<String>, 
    audio: &EditorAudio,
    dt: f64
) {
    if text.is_empty() {
        cursor.xy.0 = 0;
        cursor.xy.1 = 0;
        return;
    }

    // Clamp cursor to valid line
    cursor.xy.1 = cursor.xy.1.min(text.len() - 1);
    cursor.xy.0 = cursor.xy.0.min(text[cursor.xy.1].len());
    
    let line_len = text[cursor.xy.1].len();
    let left_steps_to_whitespace = calibrate_distance_to_whitespace_or_character(false, cursor.xy.0, &text[cursor.xy.1]);
    let right_steps_to_whitespace = calibrate_distance_to_whitespace_or_character(true, cursor.xy.0, &text[cursor.xy.1]);

    if cursor.is_combo_active(KeyCode::Left, None, dt) {
        if cursor.xy.0 > 0 {
            cursor.xy.0 = cursor.xy.0.saturating_sub(left_steps_to_whitespace);
        } else if cursor.xy.1 > 0 {
            cursor.xy.1 -= 1;
            cursor.xy.0 = text[cursor.xy.1].len();
        }

        audio.play_nav();
    }

    if cursor.is_combo_active(KeyCode::Right, None, dt) {
        if cursor.xy.0 < line_len {
            cursor.xy.0 += right_steps_to_whitespace.min(line_len - cursor.xy.0);
        } else if cursor.xy.1 + 1 < text.len() {
            cursor.xy.1 += 1;
            cursor.xy.0 = 0;
        }

        audio.play_nav();
    }
    
    // Vertical step
    let cursor_vertical_step = 5; 

    if cursor.is_combo_active(KeyCode::Up, None, dt) {
        if cursor.xy.1 > cursor_vertical_step {
            cursor.xy.1 -= cursor_vertical_step;
            cursor.xy.0 = cursor.xy.0.min(text[cursor.xy.1].len());
        } else {
            cursor.xy.1 = 0;
        }
        
        audio.play_nav();
    }

    if cursor.is_combo_active(KeyCode::Down, None, dt) {
        if cursor.xy.1 + cursor_vertical_step < text.len() {
            cursor.xy.1 += cursor_vertical_step;
            cursor.xy.0 = cursor.xy.0.min(text[cursor.xy.1].len());
        } else {
            cursor.xy.1 = text.len() - 1;
        }
            
        audio.play_nav();
    }

    recognize_cursor_word(cursor, &text[cursor.xy.1]);
}

/// Calculate the distance from the left or right 
/// to a whitepsace based on the cursor's position
/// return the distance
pub fn calibrate_distance_to_whitespace(
    leftorright: bool,
    cursor_idx: usize,
    line: &str,    
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
        
        for i in cursor..len {
            if chars[i] == ' ' {
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
        
        while cursor > 0 {
            if chars[cursor - 1] == ' ' {
                break;
            }
            
            cursor -= 1;
            steps += 1;
        }
         
        steps + 1
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
