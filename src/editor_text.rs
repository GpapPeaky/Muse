// Editor text manipulation and rendering 
// using a regex pattern to match highlight 
// colouring.
//
// The editor's identation can be switched on/off
// through the console.

use macroquad::prelude::*;

use crate::editor_audio::EditorAudio;

use crate::editor_console::editor_file::EditorFileSystem;
use crate::editor_cursor::*;

use crate::editor_console::EditorConsole;

#[path = "editor_cursor.rs"]
mod editor_cursor;

#[path = "editor_pallete.rs"]
mod editor_pallete;
use editor_pallete::*;

pub struct EditorGeneralTextStylizer {
    pub font: Font,
    pub font_size: u16,
    pub color: Color
}

impl EditorGeneralTextStylizer {
    pub async fn new() -> EditorGeneralTextStylizer {
        EditorGeneralTextStylizer {
            font: load_ttf_font("assets/font/default.ttf").await.unwrap(),
            font_size: 25,
            color: WHITE
        }
    }

    fn draw(&self, text: &str, x: f32, y: f32){
        draw_text_ex(text, x, y,
            TextParams { font: Some(&self.font), font_size: self.font_size, color: self.color, ..Default::default() });
    }
}

pub const MODE_FONT_SIZE: f32 = 30.0;
pub const MODE_Y_MARGIN: f32 = 10.0;

pub const FILE_LINE_NUMBER_X_MARGIN: f32 = 5.0;
pub const FILE_LINE_NUMBER_Y_MARGIN: f32 = 26.0;

pub const FILE_TEXT_X_MARGIN: f32 = 50.0;
pub const FILE_TEXT_Y_MARGIN: f32 = 80.0;
const TAB_SIZE: usize = 3;
const TAB_PATTERN: &str = "   ";

const CONTROL_FLOW_STATEMENTS: [&str; 46] = [
    "if", "else", "switch", "case", "default",
    "for", "while", "do", "break", "continue",
    "goto", "return", "try", "catch", "finally",
    "throw", "throws", "loop", "match", "yield",
    "await", "async", "then", "except", "raise",
    "elif", "when", "until", "unless", "foreach",
    "in", "from", "select", "where", "defer",
    "guard", "assert", "panic", "recover",
    "next", "redo", "exit", "abort", "with",
    "elif", "end",
];

const STORAGE_CLASS_SPECIFIERS: [&str; 18] = [
    "auto", "static", "extern", "register", "typedef",
    "mutable", "constexpr", "thread_local", "let", "var",
    "const", "final", "override", "sealed", "lazy",
    "owned", "borrowed", "inline",
];

const TYPE_QUALIFIERS: [&str; 14] = [
    "const", "volatile", "restrict", "constexpr",
    "ref", "mut", "transient", "synchronized",
    "abstract", "readonly", "immutable", "dynamic",
    "weak", "unsafe",
];

const COMPOSITE_TYPES: [&str; 12] = [
    "struct", "union", "enum", "class", "trait",
    "interface", "protocol", "record", "object",
    "impl", "concept", "module",
];

const MISC: [&str; 39] = [
    "sizeof", "inline", "virtual", "explicit", "namespace",
    "using", "operator", "template", "typename", "friend",
    "crate", "super", "self", "import", "package",
    "include", "public", "private", "protected", "internal",
    "static_cast", "reinterpret_cast", "dynamic_cast", "const_cast",
    "typeof", "instanceof", "new", "delete", "clone",
    "as", "is", "extends", "implements", "default",
    "partial", "module", "export", "require", "use",
];

const DATA_TYPES: [&str; 60] = [
    "int", "float", "double", "char", "void",
    "short", "long", "signed", "unsigned", "bool",
    "boolean", "byte", "wchar_t", "auto", "decltype",
    "nullptr_t", "String", "str", "u8", "u16",
    "u32", "u64", "u128", "i8", "i16", "i32",
    "i64", "i128", "f32", "f64", "usize", "isize",
    "any", "object", "None", "null", "undefined",
    "map", "list", "array", "tuple", "set", "dict",
    "Vec", "Option", "Result", "number", "char8_t",
    "char16_t", "char32_t", "interface", "record", "trait",
    "enum", "struct", "unit", "string", "symbol",
    "function", "object",
];

/// Convert a provided character index to the actual byte
/// the character is at. Allows for UTF-8 characters
/// and not only ASCII
pub fn char_to_byte(line: &str, char_idx: usize) -> usize {
    // We use UTF-8 so we need to count bytes NOT characters like C.
    line.char_indices().nth(char_idx).map(|(b, _)| b).unwrap_or(line.len())
}

/// Calibrate the color of a token
fn calibrate_string_color(string: &str) -> Color {
    if CONTROL_FLOW_STATEMENTS.contains(&string) {
        return CONTROL_FLOW_COLOR;
    } else if TYPE_QUALIFIERS.contains(&string) {
        return TYPE_QUALIFIER_COLOR;
    } else if COMPOSITE_TYPES.contains(&string) {
        return COMPOSITE_TYPE_COLOR;
    } else if STORAGE_CLASS_SPECIFIERS.contains(&string) {
        return STORAGE_CLASS_COLOR;
    } else if MISC.contains(&string) {
        return MISC_COLOR;
    } else if DATA_TYPES.contains(&string) {
        return DATA_TYPE_COLOR;
    } else if string.chars().all(|c| c.is_ascii_digit()) {
        return NUMBER_LITERAL_COLOR;
    } else {
        return IDENTIFIER_COLOR;
    }
}

/// Record special key presses
pub fn record_special_keys(cursor: &mut EditorCursor, text: &mut Vec<String>, audio: &EditorAudio, console: &mut EditorConsole, gts: &mut EditorGeneralTextStylizer, efs: &mut EditorFileSystem) -> bool {
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

    if is_key_pressed(KeyCode::Tab) {
        audio.play_space();

        let line = &mut text[cursor.xy.1];
        let byte_idx = char_to_byte(line, cursor.xy.0);
        line.insert_str(byte_idx, TAB_PATTERN);
        cursor.xy.0 += TAB_SIZE;

        efs.unsaved_changes = true;
        return true;
    }

    if is_key_pressed(KeyCode::Enter) {
        audio.play_return();

        let line = &mut text[cursor.xy.1];
        let rest = line.split_off(char_to_byte(line, cursor.xy.0));
        cursor.xy.1 += 1;

        // TODO: Smarter identation here

        cursor.xy.0 = 0;
        
        text.insert(cursor.xy.1, rest);

        efs.unsaved_changes = true;
        return true;
    }

    // More special keys
    if is_key_down(KeyCode::LeftControl) {
        // Console switch
        if is_key_pressed(KeyCode::GraveAccent) {
            console.mode = true; 
        }

        if is_key_pressed(KeyCode::Minus) {
            if gts.font_size > 12 {
                gts.font_size -= 2;
            }
        }
        
        if is_key_pressed(KeyCode::Equal) {
            if gts.font_size < 45 {
                gts.font_size += 2;
            }
        }

        file_text_special_navigation(&mut cursor.xy, text, audio);

        return true;
    } else {
        file_text_navigation(&mut cursor.xy, text, audio);
    }

    false
}

/// Standard key recording function
pub fn record_keyboard_to_file_text(cursor: &mut EditorCursor, text: &mut Vec<String>, audio: &EditorAudio, console: &mut EditorConsole, gts: &mut EditorGeneralTextStylizer, efs: &mut EditorFileSystem) {
    // let c = get_char_pressed().unwrap(); // Unwrap removes the Result/Option wrapper.

    if text.is_empty() { // Allocate memory for a new string
        text.push(String::new());
    }

    if record_special_keys(cursor, text, audio, console, gts, efs) {
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
            // '\u{8}' | '\r' | '\n' | '\t' => {
            //     // We also have to pre-terminate with these special characters,
            //     // since input is passed in a queue
            //     return; // Special characters will be handled elsewhere
            // }

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

// FIXME: Optimize.
/// Text drawing function
pub fn draw(text: &Vec<String>, cursor_x: usize, cursor_y: usize, gts: &mut EditorGeneralTextStylizer, console: &EditorConsole, camera: &mut crate::editor_camera::EditorCamera) {
    // Occlusion culling
    let cam_left = camera.offset_x;
    let cam_right = camera.offset_x + screen_width();
    let cam_top = camera.offset_y;
    let cam_bottom = camera.offset_y + screen_height();
    
    let start_x = FILE_TEXT_X_MARGIN;
    let start_y = FILE_TEXT_Y_MARGIN;
    let line_spacing = gts.font_size as f32;
    let line_start_relative_to_font_size_fix = gts.font_size as f32 * 1.5;

    // Draw cursor
    if !console.mode && cursor_y < text.len() {
        let line = &text[cursor_y];
        let prefix = &line[..cursor_x];
        let text_before_cursor = measure_text(prefix, Some(&gts.font), gts.font_size, 1.0);
        let cursor_y_pos = start_y + cursor_y as f32 * line_spacing;
        let cursor_x_pos = start_x + line_start_relative_to_font_size_fix + text_before_cursor.width;
        
        camera.follow_cursor(cursor_x_pos, cursor_y_pos);
        
        // For cursor width, just measure next char if exists
        let cursor_width = if cursor_x < line.len() {
            measure_text(&line[cursor_x..].chars().next().unwrap().to_string(), Some(&gts.font), gts.font_size, 1.0).width
        } else {
            2.0
        };

        let (sx, sy) = camera.world_to_screen(cursor_x_pos, cursor_y_pos - gts.font_size as f32 * 0.8 + 25.0);
        draw_rectangle(
            sx,
            sy,
            cursor_width,
            gts.font_size as f32,
            CURSOR_COLOR,
        );
    }

    let mut x;
    let mut y;

    let mut in_string = false;
    let mut in_block_comment = false;

    for (line_index, line) in text.iter().enumerate() {
        y = start_y + line_index as f32 * line_spacing;

        // Skip lines not in camera
        if y + line_spacing < cam_top || y > cam_bottom {
            continue;
        }
        
        x = start_x + line_start_relative_to_font_size_fix;
        let mut chars = line.chars().peekable();
        while let Some(&c) = chars.peek() {
            #[allow(unused_assignments)]
            let mut color = IDENTIFIER_COLOR;
            let mut token = String::new();

            if in_block_comment {
                // Inside multiline comment
                while let Some(ch) = chars.next() {
                    token.push(ch);
                    if ch == '*' && chars.peek() == Some(&'/') {
                        token.push(chars.next().unwrap());
                        in_block_comment = false;
                        break;
                    }
                }
                color = COMMENT_COLOR;
            } else if in_string {
                // Inside string
                while let Some(ch) = chars.next() {
                    token.push(ch);
                    if ch == '"' && !token.ends_with("\\\"") {
                        in_string = false;
                        break;
                    }
                }
                color = STRING_LITERAL_COLOR;
            } else {
                // Not in comment or string
                match c {
                    '/' => {
                        chars.next();
                        if chars.peek() == Some(&'/') {
                            chars.next();
                            token.push_str("//");
                            token.extend(chars.by_ref());
                            color = COMMENT_COLOR;
                        } else if chars.peek() == Some(&'*') {
                            chars.next();
                            token.push_str("/*");
                            in_block_comment = true;
                            color = COMMENT_COLOR;
                        } else {
                            token.push('/');
                            color = PUNCTUATION_COLOR;
                        }
                    }
                    '"' => {
                        chars.next();
                        token.push('"');
                        in_string = true;
                        color = STRING_LITERAL_COLOR;
                    }
                    '#' => {
                        // Macro: consume until whitespace ends token
                        while let Some(&ch) = chars.peek() {
                            if ch.is_whitespace() {
                                break;
                            }
                            token.push(chars.next().unwrap());
                        }
                        color = MACRO_COLOR;

                        // Also consume subsequent tokens for spaced macros
                        while let Some(&ch) = chars.peek() {
                            if !ch.is_whitespace() {
                                break;
                            }
                            token.push(chars.next().unwrap());
                            while let Some(&ch2) = chars.peek() {
                                if ch2.is_whitespace() {
                                    break;
                                }
                                token.push(chars.next().unwrap());
                            }
                        }
                    }
                    '<' => {
                        // Only color as string in #include lines
                        if line.trim_start().starts_with("#include") {
                            chars.next();
                            token.push('<');
                            while let Some(ch) = chars.next() {
                                token.push(ch);
                                if ch == '>' {
                                    break;
                                }
                            }
                            color = STRING_LITERAL_COLOR;
                        } else {
                            token.push(chars.next().unwrap());
                            color = PUNCTUATION_COLOR;
                        }
                    }
                    c if c.is_whitespace() => {
                        while let Some(&ch) = chars.peek() {
                            if !ch.is_whitespace() {
                                break;
                            }
                            token.push(chars.next().unwrap());
                        }
                        color = IDENTIFIER_COLOR;
                    }
                    c if c.is_ascii_digit() => {
                        while let Some(&ch) = chars.peek() {
                            if !(ch.is_ascii_digit() || ch == '.' || ch == 'f' || ch == 'F' || ch == '-') {
                                break;
                            }
                            token.push(chars.next().unwrap());
                        }
                        color = NUMBER_LITERAL_COLOR;
                    }
                    c if !c.is_alphanumeric() && c != '_' => {
                        token.push(chars.next().unwrap());
                        color = PUNCTUATION_COLOR;
                    }
                    _ => {
                        while let Some(&ch) = chars.peek() {
                            if !ch.is_alphanumeric() && ch != '_' {
                                break;
                            }
                            token.push(chars.next().unwrap());
                        }
                        let clean = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                        color = calibrate_string_color(clean);
                    }
                }
            }

            let rough_width = token.len() as f32 * gts.font_size as f32 * 0.5;

            if x + rough_width < cam_left || x > cam_right { 
                x += rough_width; 
                continue; 
            }

            // Measure only if inside the camera
            let token_width = measure_text(&token, Some(&gts.font), gts.font_size, 1.0).width;

            gts.color = color;
            let (sx, sy) = camera.world_to_screen(x, y + 25.0);
            gts.draw(&token, sx, sy);

            x += token_width;
        }
    }

    // Line side bar
    let x_offset = 5.0;
    draw_rectangle(0.0, 0.0, start_x + line_start_relative_to_font_size_fix - x_offset, screen_height(), COMPOSITE_TYPE_COLOR);
    draw_rectangle(0.0, 0.0, start_x + line_start_relative_to_font_size_fix - x_offset - 1.0, screen_height(), BACKGROUND_COLOR);

    // Draw line numbers
    gts.color = CURSOR_COLOR;

    let first_visible_line = ((cam_top - start_y) / line_spacing).max(0.0) as usize;
    let last_visible_line = ((cam_bottom - start_y) / line_spacing).min(text.len() as f32 - 1.0) as usize;
    for i in first_visible_line..=last_visible_line {
        let line_y_world = 1.1 * FILE_TEXT_X_MARGIN + FILE_LINE_NUMBER_Y_MARGIN + gts.font_size as f32 * i as f32 + 25.0;
    
        let screen_y = line_y_world - camera.offset_y;
        
        gts.draw(
            &i.to_string(),
            FILE_LINE_NUMBER_X_MARGIN,
            screen_y,
        );
    }

    // Top bar
    draw_rectangle(0.0, 0.0, screen_width(), MODE_Y_MARGIN + MODE_FONT_SIZE + 25.0 + 1.0, COMPOSITE_TYPE_COLOR);
    draw_rectangle(0.0, 0.0, screen_width(), MODE_Y_MARGIN + MODE_FONT_SIZE + 25.0, BACKGROUND_COLOR);

    if console.mode {
        console.draw();
    }
}
