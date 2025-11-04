// Editor text manipulation and rendering 
// using a regex pattern to match highlight 
// colouring.
//
// The editor's identation can be switched on/off
// through the console.

use macroquad::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::editor_audio::EditorAudio;

use crate::editor_cursor::*;

use crate::editor_console::EditorConsole;

#[path = "editor_cursor.rs"]
mod editor_cursor;

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

// Regex pattern order matters: comments, strings, numbers, words, punctuation, whitespace
static TOKEN_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"//[^\n]*|/\*.*?\*/|"(?:\\.|[^"\\])*"|<[^>\n]+>|\b\d+(\.\d+)?([fF]\b)?\b|#[\w_]+|[\w\*]+|[^\w\s]+|\s+"#
    ).unwrap()
});

pub const MODE_FONT_SIZE: f32 = 30.0;
pub const MODE_Y_MARGIN: f32 = 10.0;

const FILE_LINE_NUMBER_X_MARGIN: f32 = 5.0;
const FILE_LINE_NUMBER_Y_MARGIN: f32 = 26.0;

const FILE_TEXT_X_MARGIN: f32 = 50.0;
const FILE_TEXT_Y_MARGIN: f32 = 80.0;
const TAB_SIZE: usize = 6;
const TAB_PATTERN: &str = "      ";

pub const BACKGROUND_COLOR: Color     = Color::from_rgba(25, 0, 20, 255);       // Wine velvet — intimate depth
const IDENTIFIER_COLOR: Color         = Color::from_rgba(247, 180, 10, 255);
const PUNCTUATION_COLOR: Color        = Color::from_rgba(225, 185, 135, 255);
const CONTROL_FLOW_COLOR: Color       = Color::from_rgba(255, 80, 180, 255);    // Fuchsia pulse — flowing emotion
pub const STORAGE_CLASS_COLOR: Color  = Color::from_rgba(170, 0, 80, 255);      // Crimson wine — deep intensity
const TYPE_QUALIFIER_COLOR: Color     = Color::from_rgba(255, 190, 150, 255);   // Peach — warmth and tenderness
pub const COMPOSITE_TYPE_COLOR: Color = Color::from_rgba(200, 90, 255, 255);    // Violet silk — romantic complexity
const MISC_COLOR: Color               = Color::from_rgba(255, 110, 130, 255);   // Coral — playful affection
const DATA_TYPE_COLOR: Color          = Color::from_rgba(230, 120, 90, 255);
const NUMBER_LITERAL_COLOR: Color     = Color::from_rgba(255, 230, 205, 255);
const STRING_LITERAL_COLOR: Color     = Color::from_rgba(240, 5, 5, 255);
const COMMENT_COLOR: Color            = Color::from_rgba(255, 100, 0, 255);
const CURSOR_COLOR: Color             = Color::from_rgba(255, 200, 190, 255);
const MACRO_COLOR: Color              = Color::from_rgba(255, 60, 100, 255);    // Candy red — invitation
const MAIN_COLOR: Color               = Color::from_rgba(200, 120, 145, 255); 

const C_CONTROL_FLOW_STATEMENTS: [&str ; 12] = [
    "if",
    "else",
    "switch",
    "case",
    "default",
    "for",
    "while",
    "do",
    "break",
    "continue",
    "goto",
    "return"
];

const C_STORAGE_CLASS_SPECIFIERS: [&str ; 5] = [
    "auto",
    "static",
    "extern",
    "register",
    "typedef"
];

const C_TYPE_QUALIFIERS: [&str ; 1] = [
    "const"
];

const C_COMPOSITE_TYPES: [&str ; 3] = [
    "struct",
    "union",
    "enum"
];

const C_MISC: [&str ; 2] = [
    "sizeof",
    "inline"
];

const C_DATA_TYPES: [&str ; 9] = [
    "int",
    "float",
    "double",
    "char",
    "void",
    "short",
    "long",
    "signed",
    "unsigned"
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
    if C_CONTROL_FLOW_STATEMENTS.contains(&string) {
        return CONTROL_FLOW_COLOR;
    } else if C_TYPE_QUALIFIERS.contains(&string) {
        return TYPE_QUALIFIER_COLOR;
    } else if C_COMPOSITE_TYPES.contains(&string) {
        return COMPOSITE_TYPE_COLOR;
    } else if C_STORAGE_CLASS_SPECIFIERS.contains(&string) {
        return STORAGE_CLASS_COLOR;
    } else if C_MISC.contains(&string) {
        return MISC_COLOR;
    } else if C_DATA_TYPES.contains(&string) {
        return DATA_TYPE_COLOR;
    } else if string.chars().all(|c| c.is_ascii_digit()) {
        return NUMBER_LITERAL_COLOR;
    } else {
        return IDENTIFIER_COLOR;
    }
}

/// Record special key presses
pub fn record_special_keys(cursor: &mut EditorCursor, text: &mut Vec<String>, audio: &EditorAudio, console: &mut EditorConsole, gts: &mut EditorGeneralTextStylizer) -> bool {
    if is_key_pressed(KeyCode::Backspace) {
        audio.play_delete();

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
pub fn record_keyboard_to_file_text(cursor: &mut EditorCursor, text: &mut Vec<String>, audio: &EditorAudio, console: &mut EditorConsole, gts: &mut EditorGeneralTextStylizer) {
    // let c = get_char_pressed().unwrap(); // Unwrap removes the Result/Option wrapper.

    if text.is_empty() { // Allocate memory for a new string
        text.push(String::new());
    }

    if record_special_keys(cursor, text, audio, console, gts) {
        return; // Handle the special key and terminate the call, as to 
        // not record any special escape character
    }

    if let Some(c) = get_char_pressed() {
        // We will also handle smart/smarter identation here.
        while cursor.xy.1 >= text.len() {
            text.push(String::new());
        }
        match c {
            '\u{8}' | '\r' | '\n' | '\t' => {
                // We also have to pre-terminate with these special characters,
                // since input is passed in a queue
                return; // Special characters will be handled elsewhere
            }

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

/// Text drawing function
pub fn draw(text: &Vec<String>, cursor_x: usize, cursor_y: usize, gts: &mut EditorGeneralTextStylizer, console: &EditorConsole) {
    let start_x = FILE_TEXT_X_MARGIN;
    let start_y = FILE_TEXT_Y_MARGIN;
    let line_spacing = gts.font_size as f32;
    let line_start_relative_to_font_size_fix = gts.font_size as f32 * 1.5;

    draw_line(0.0, MODE_Y_MARGIN + MODE_FONT_SIZE, screen_width(), MODE_Y_MARGIN + MODE_FONT_SIZE, 1.0, COMPOSITE_TYPE_COLOR);
    
    if !console.mode {
        // Draw text cursor
        if cursor_y < text.len() {
            let line = &text[cursor_y];
            let cursor_text = &line[..cursor_x.min(line.len())];
            let text_before_cursor = measure_text(cursor_text, Some(&gts.font), gts.font_size, 1.0);
            let cursor_x_pos = start_x + line_start_relative_to_font_size_fix + text_before_cursor.width;
            let cursor_y_pos = start_y + cursor_y as f32 * line_spacing;

            // Cursor width, either of the current char size, or static 2.0px
            let cursor_width = if true && cursor_x < line.len() { // Here CURSOR_LINE_TO_WIDTH was used here
                measure_text(
                    &line.chars().nth(cursor_x).unwrap().to_string(),
                    Some(&gts.font),
                    gts.font_size,
                    1.0,
                ).width
            } else {
                2.0
            };

            draw_rectangle(
                cursor_x_pos,
                cursor_y_pos - gts.font_size as f32 * 0.8,
                cursor_width,
                gts.font_size as f32,
                CURSOR_COLOR,
            );
        }
    } else {

    }

    let mut x;
    let mut y;    

    for (line_index, line) in text.iter().enumerate() {
        x = start_x + line_start_relative_to_font_size_fix;
        y = start_y + line_index as f32 * line_spacing;

        for cap in TOKEN_PATTERN.find_iter(line) {
            let token = cap.as_str();

            let color = if token.starts_with("//") || token.starts_with("/*") {
                COMMENT_COLOR
            } else if token.trim_start().starts_with("#") {
                MACRO_COLOR
            } else if (token.starts_with('"') && token.ends_with('"')) || (token.starts_with('<') && token.ends_with('>')) {
                STRING_LITERAL_COLOR
            } else if token.chars().all(|c| c.is_whitespace()) {
                IDENTIFIER_COLOR
            } else if token.chars().all(|c| !c.is_alphanumeric() && !c.is_whitespace() && c != '_') {
                PUNCTUATION_COLOR
            } else if TOKEN_PATTERN.is_match(token) && token.chars().any(|c| c.is_ascii_digit()) {
                NUMBER_LITERAL_COLOR
            } else if token == "main" {
                MAIN_COLOR
            } else {
                // Normal identifiers like variable names and functions
                let clean = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                calibrate_string_color(clean)
            };

            // FIXME Negative number colouring with a '-' is colored as a punctuation
            // FIXME Strings inside (str) are not coloured properly.
            // FIXME Strings broken by newlines are not colored properly.
            // FIXME Macros when brocken by white space, not colored properly.
            // FIXME Numbers inside identifiers, get coloured as numbers
            // FIXME Some control characters still show

            // Draw token at once using the general text stylizer
            gts.color = color;
            gts.draw(token, x, y);
            // draw_text(token, x, y, gts.font_size, color);

            // More effective cursor movement
            // Avoid cursor x/y calibration per character
            let token_width = measure_text(token, Some(&gts.font), gts.font_size, 1.0).width;
            x += token_width;
        }
    }

    // Draw line numbers
    gts.color = CURSOR_COLOR;

    let text_len;
    if text.is_empty() {
        text_len = 0;
    } else {
        text_len = text.len();
    }

    for i in 0..text_len {
        gts.draw(&i.to_string(), FILE_LINE_NUMBER_X_MARGIN,
            1.1 * FILE_TEXT_X_MARGIN + FILE_LINE_NUMBER_Y_MARGIN + gts.font_size as f32 * i as f32
        );
    }

    if console.mode {
        console.draw();
    }
}
