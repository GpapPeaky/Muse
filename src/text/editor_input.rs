// Text input module

use macroquad::prelude::*;
use crate::options::editor_options::EditorOptions;
use crate::text::editor_language_manager::EditorLanguageKeywords;
use crate::text::editor_text_stylizer::*;
use crate::text::editor_cursor::*;

use crate::audio::editor_audio::*;
use crate::console::editor_console::*;
use crate::console::editor_file::*;
use crate::console::editor_directives::*;

pub const TAB_SIZE: usize = 4;
pub const TAB_PATTERN: &str = "    ";

/// Convert a provided character index to the actual byte
/// the character is at. Allows for UTF-8 characters
/// and not only ASCII
pub fn char_to_byte(
    line: &str,
    char_idx: usize
) -> usize {
    // We use UTF-8 so we need to count bytes NOT characters like C.
    line.char_indices().nth(char_idx).map(|(b, _)| b).unwrap_or(line.len())
}

/// Left control shortcuts
fn lctrl_shortcuts(
    cursor: &mut EditorCursor,
    text: &mut Vec<String>,
    audio: &EditorAudio,
    console: &mut EditorConsole,
    efs: &mut EditorFileSystem,
    gts: &mut EditorGeneralTextStylizer,
    ops: &mut EditorOptions,
    elk: &mut EditorLanguageKeywords
) -> bool {
    // Left control shorcuts
    if is_key_down(KeyCode::LeftControl) {
        // Specific to general.

        // Delete line
        if is_key_pressed(KeyCode::X) {
            if text.len() > 0 {
                audio.play_delete();
                efs.unsaved_changes = true;
                text.remove(cursor.xy.1);
            }

            return true;
        }

        // Save/write to file
        if is_key_pressed(KeyCode::S) {
            console.directive = ":w".to_string();
            execute_directive(&mut console.directive, efs, text, cursor, ops, elk);

            return true;
        }
        
        // Go to line
        if is_key_pressed(KeyCode::L) {
            console.directive = ":l ".to_string();
            console.mode = true;
            // Opens the console with the cursor right on where it needs to be
            console.cursor.x = console.directive.len();

            return true;
        }

        // Open native file explorer        
        if is_key_pressed(KeyCode::O) {
            console.directive = ":O".to_string();
            execute_directive(&mut console.directive, efs, text, cursor, ops, elk);

            return true;
        }
        
        // Create a new file
        if is_key_pressed(KeyCode::N) {
            console.directive = ":c f".to_string();
            execute_directive(&mut console.directive, efs, text, cursor, ops, elk);
            console.directive = ":b ".to_string();
            console.mode = true;
            console.cursor.x = console.directive.len();
        
            return true;
        }
        
        // 'Baptize' current file
        if is_key_pressed(KeyCode::B) {
            console.directive = ":b ".to_string();
            console.mode = true;
            console.cursor.x = console.directive.len();

            return true;
        }
        
        // Remove current file
        if is_key_pressed(KeyCode::R) {
            console.directive = ":r".to_string();
            execute_directive(&mut console.directive, efs, text, cursor, ops, elk);

            return true;
        }
        
        // Create directory
        if is_key_pressed(KeyCode::M) {
            console.directive = ":md ".to_string();
            console.mode = true;
            console.cursor.x = console.directive.len();

            return true;
        }

        // Duplicate line
        if is_key_pressed(KeyCode::D) {
            if text.len() > 0 {
                audio.play_insert();
                let line_clone = text[cursor.xy.1].clone();
                text.insert(cursor.xy.1 + 1, line_clone);
            }
    
            return true;
        }
        
        // Delete the word that the cursor is currently at
        if is_key_pressed(KeyCode::W) {
            // Find the character collection of the word, left and right
            // from the word_idx

            let cursor_idx = cursor.xy.0;
            let left_distance = calibrate_distance_to_whitespace(false, cursor_idx, &text[cursor.xy.1]);
            let right_distance = calibrate_distance_to_whitespace(true, cursor_idx, &text[cursor.xy.1]);
            
            // Index of where the word starts
            let left_cursor_idx = cursor_idx - left_distance;
            
            // Word length.
            let word_len = cursor_idx + right_distance;
            
            // Actual deletion
            for _ in left_cursor_idx..word_len {
                let line = &mut text[cursor.xy.1];
                let byte_idx = char_to_byte(line, left_cursor_idx);
                if byte_idx < line.len() {
                    line.remove(byte_idx);
                }
            }
            
            audio.play_delete();
            efs.unsaved_changes = true;
            
            return true;
        }
        
        // Save and quit
        if is_key_pressed(KeyCode::Q) {
            console.directive = ":W".to_string();
            execute_directive(&mut console.directive, efs, text, cursor, ops, elk);
            console.directive = ":q".to_string();
            execute_directive(&mut console.directive, efs, text, cursor, ops, elk);
        }
        
        // Quit
        if is_key_pressed(KeyCode::E) {
            console.directive = ":e".to_string();
            execute_directive(&mut console.directive, efs, text, cursor, ops, elk);
        }

        // Console switch
        if is_key_pressed(KeyCode::GraveAccent) {
            console.mode = true; 

            return true;
        }

        if is_key_pressed(KeyCode::Minus) {
            if gts.font_size > 12 {
                gts.font_size -= 2;

            }

            return true;
        }
        
        if is_key_pressed(KeyCode::Equal) {
            if gts.font_size < 45 {
                gts.font_size += 2;

            }

            return true;
        }
        
        file_text_special_navigation(cursor, text, audio);

        return true;
    }

    false
}

/// Record special key presses
pub fn record_special_keys(
    cursor: &mut EditorCursor,
    text: &mut Vec<String>,
    audio: &EditorAudio,
    console: &mut EditorConsole,
    gts: &mut EditorGeneralTextStylizer,
    efs: &mut EditorFileSystem,
    ops: &mut EditorOptions,
    elk: &mut EditorLanguageKeywords
) -> bool {
    // Backspace
    if is_key_pressed(KeyCode::Backspace) {
        audio.play_delete();
        efs.unsaved_changes = true;

        if text.is_empty() {
            return true;
        }
    
        // Clamp cursor_x to line length
        let line = &mut text[cursor.xy.1];
        let line_len = line.chars().count();
        cursor.xy.0 = (cursor.xy.0).min(line_len);
    
        if cursor.xy.0 == 0 {
            // Merge with previous line if possible
            if cursor.xy.1 > 0 {
                let current_line = text.remove(cursor.xy.1);
                cursor.xy.1 -= 1;
                cursor.xy.0 = text[cursor.xy.1].chars().count();
                text[cursor.xy.1].push_str(&current_line);
            }


            return true;
        }
    
        let cursor_pos = cursor.xy.0;
    
        // Tab deletion
        if cursor_pos >= TAB_SIZE {
            let start_char = cursor_pos - TAB_SIZE;
            let end_char = cursor_pos;
            let start_byte = char_to_byte(line, start_char);
            let end_byte = char_to_byte(line, end_char);
    
            if &line[start_byte..end_byte] == TAB_PATTERN {
                line.replace_range(start_byte..end_byte, "");
                cursor.xy.0 -= TAB_SIZE;

                return true;
            }
        }
    
        // Normal deletion
        let byte_idx = char_to_byte(line, cursor_pos - 1);
        if byte_idx < line.len() {
            line.remove(byte_idx);
            cursor.xy.0 -= 1;
        }

        return true;
    }

    // Tab insertion
    if is_key_pressed(KeyCode::Tab) {
        audio.play_space();

        let line = &mut text[cursor.xy.1];
        let byte_idx = char_to_byte(line, cursor.xy.0);
        line.insert_str(byte_idx, TAB_PATTERN);
        
        cursor.xy.0 += TAB_SIZE;

        return true;
    }

    // Return/Enter key
    if is_key_pressed(KeyCode::Enter) {
        audio.play_return();
        efs.unsaved_changes = true;
    
        let line = &mut text[cursor.xy.1];
        let cursor_pos = cursor.xy.0;
    
        // Split the line at cursor
        let rest_of_line = line.split_off(char_to_byte(line, cursor_pos));
    
        // Get base indentation (spaces/tabs at start of line)
        let base_indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
    
        // Determine if we should increase indent
        let increase_indent = matches!(line.trim_end().chars().last(), Some('{') | Some('(') | Some('['));
    
        // Determine if we need to insert a closer
        let next_closer = match (line.trim_end().chars().last(), rest_of_line.chars().next()) {
            (Some('{'), Some('}')) => Some('}'),
            (Some('('), Some(')')) => Some(')'),
            (Some('['), Some(']')) => Some(']'),
            _ => None,
        };
    
        // Prepare new line indentation
        let mut new_line = base_indent.clone();
        if increase_indent {
            new_line.push_str(TAB_PATTERN);
        }
    
        // Insert new line
        cursor.xy.1 += 1;
        cursor.xy.0 = new_line.chars().count();
        text.insert(cursor.xy.1, new_line);
    
        // If there’s a closer, handle it smartly
        if let Some(closer) = next_closer {
            // Remove the closer from rest_of_line
            let rest_cleaned = rest_of_line[closer.len_utf8()..].to_string();
            text[cursor.xy.1 - 1].push_str(&rest_cleaned); // append rest to previous line
            text.insert(cursor.xy.1 + 1, format!("{}{}", base_indent, closer)); // insert closer on new line
        } else if !rest_of_line.is_empty() {
            text.insert(cursor.xy.1 + 1, rest_of_line);
        }
    }
        
    if !lctrl_shortcuts(cursor, text, audio, console, efs, gts, ops, elk) {
        file_text_navigation(cursor, text, audio);
    }

    false
}

/// Standard key recording function
pub fn record_keyboard_to_file_text(
    cursor: &mut EditorCursor,
    text: &mut Vec<String>,
    audio: &EditorAudio,
    console: &mut EditorConsole,
    gts: &mut EditorGeneralTextStylizer,
    efs: &mut EditorFileSystem,
    ops: &mut EditorOptions,
    elk: &mut EditorLanguageKeywords
) {
    // let c = get_char_pressed().unwrap(); // Unwrap removes the Result/Option wrapper.

    if text.is_empty() { // Allocate memory for a new string
        text.push(String::new());
    }

    if record_special_keys(cursor, text, audio, console, gts, efs, ops, elk) {
        return; // Handle the special key and terminate the call, as to 
        // not record any special escape character
    }

    if let Some(c) = get_char_pressed() {
        // Skip control characters
        if c.is_control() || c.is_ascii_control() {
            return;
        }
    
        efs.unsaved_changes = true;

        // We will also handle smart/smarter identation here.
        while cursor.xy.1 >= text.len() {
            text.push(String::new());
        }
        match c {
            '<' => {
                audio.play_insert();

                let line = &mut text[cursor.xy.1];

                let byte_idx = char_to_byte(line, cursor.xy.0);
                
                line.insert(byte_idx, c);
                
                cursor.xy.0 += 1;
                
                let next_byte_idx = char_to_byte(line, cursor.xy.0);

                line.insert(next_byte_idx, '>');
            }

            '(' => {
                audio.play_insert();

                let line = &mut text[cursor.xy.1];

                let byte_idx = char_to_byte(line, cursor.xy.0);
                
                line.insert(byte_idx, c);
                
                cursor.xy.0 += 1;
                
                let next_byte_idx = char_to_byte(line, cursor.xy.0);

                line.insert(next_byte_idx, ')');
            }

            '{' => {
                audio.play_insert();

                let line = &mut text[cursor.xy.1];

                let byte_idx = char_to_byte(line, cursor.xy.0);
                
                line.insert(byte_idx, c);
                
                cursor.xy.0 += 1;
                
                let next_byte_idx = char_to_byte(line, cursor.xy.0);

                line.insert(next_byte_idx, '}');
            }

            '\'' => {
                audio.play_insert();

                let line = &mut text[cursor.xy.1];

                let byte_idx = char_to_byte(line, cursor.xy.0);
                
                line.insert(byte_idx, c);
                
                cursor.xy.0 += 1;
                
                let next_byte_idx = char_to_byte(line, cursor.xy.0);

                line.insert(next_byte_idx, '\'');
            }

            '"' => {
                audio.play_insert();

                let line = &mut text[cursor.xy.1];

                let byte_idx = char_to_byte(line, cursor.xy.0);
                
                line.insert(byte_idx, c);
                
                cursor.xy.0 += 1;
                
                let next_byte_idx = char_to_byte(line, cursor.xy.0);

                line.insert(next_byte_idx, '"');
            }

            '[' => {
                audio.play_insert();

                let line = &mut text[cursor.xy.1];

                let byte_idx = char_to_byte(line, cursor.xy.0);
                
                line.insert(byte_idx, c);
                
                cursor.xy.0 += 1;
                
                let next_byte_idx = char_to_byte(line, cursor.xy.0);

                line.insert(next_byte_idx, ']');
            }

            _ => {
                if c != ' ' { 
                    audio.play_insert();
                } else {
                    audio.play_space();
                }

                let line = &mut text[cursor.xy.1];

                let byte_idx = char_to_byte(line, cursor.xy.0);
                
                line.insert(byte_idx, c); // Normal insertion.
                cursor.xy.0 += 1;
            }
        }
    }
}
