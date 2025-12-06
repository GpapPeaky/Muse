// Editor text manipulation and rendering 
// using a regex pattern to match highlight 
// colouring.
//
// The editor's identation can be switched on/off
// through the console.

use macroquad::prelude::*;

use crate::console::editor_console::*;
use crate::options::editor_pallete::*;
use crate::text::editor_cursor::*;
use crate::text::editor_input::*;
use crate::text::editor_language_manager::EditorLanguageKeywords;
use crate::text::editor_text_stylizer::*;
use crate::camera::editor_camera::*;

pub const CURRENT_FILE_TOP_BAR_OFFSET: f32 = 100.0;

pub const MODE_FONT_SIZE: f32 = 30.0;
pub const MODE_Y_MARGIN: f32 = 10.0;
pub const MODE_Y_OFFSET: f32 = 15.0;

pub const FILE_LINE_NUMBER_X_MARGIN: f32 = 5.0;
pub const FILE_LINE_NUMBER_Y_MARGIN: f32 = 26.0;

pub const FILE_TEXT_X_MARGIN: f32 = 50.0;
pub const FILE_TEXT_Y_MARGIN: f32 = 80.0;

/// Find a word in the text 
/// and move the cursor there
/// return true if found, false if not
pub fn find_word_in_text(
    word: &str,
    text: &Vec<String>,
    cursor: &mut EditorCursor
) -> bool {
    if let Some(line_index) = text.iter().position(|line| line.contains(word)) {
        cursor.xy.1 = line_index;
        true
    } else {
        false
    }
}

/// All around draw function for the editor text
pub fn draw_file_text(
    text: &Vec<String>,
    cursor: &mut EditorCursor,
    gts: &mut EditorGeneralTextStylizer,
    console: &EditorConsole,
    camera: &mut EditorCamera,
    elk: &EditorLanguageKeywords
) {
    let text_y_offset = 25.0;

    let start_x = FILE_TEXT_X_MARGIN;
    let start_y = FILE_TEXT_Y_MARGIN;
    let line_spacing = gts.font_size as f32;
    let line_start_fix = gts.font_size as f32 * 1.5;

    // let cam_left = camera.offset_x;
    // let cam_right = camera.offset_x + screen_width();
    let cam_top = camera.offset_y;
    let cam_bottom = camera.offset_y + screen_height();

    // Draw cursor
    if !console.mode && cursor.xy.1 < text.len() {
        let line = &text[cursor.xy.1];
        let byte_idx = char_to_byte(line, cursor.xy.0);
        let prefix = &line[..byte_idx];
    
        let visual_prefix = prefix.replace("\t", TAB_PATTERN);
        let text_before_cursor = measure_text(&visual_prefix, Some(&gts.font), gts.font_size, 1.0);
    
        // Target location to draw
        let logical_x = start_x + line_start_fix + text_before_cursor.width;
        let logical_y = start_y + cursor.xy.1 as f32 * line_spacing + text_y_offset;
        
        // Smooth animation step, via interpolation
        cursor.animate_to(logical_x, logical_y);
        
        // Use animated position instead of logical
        let cursor_x_pos = cursor.anim_x;
        let cursor_y_pos = cursor.anim_y;
    
        camera.follow_cursor(cursor_x_pos, cursor_y_pos);
    
        let (sx, sy) = camera.world_to_screen(cursor_x_pos, cursor_y_pos);
        
        let cursor_width = gts.font_size as f32 / 7.5;

        let draw_x = sx.round();
        let draw_y = sy.round();
        let cursor_line_draw_y = draw_y - gts.font_size as f32;
        let cursor_line_draw_x = FILE_TEXT_X_MARGIN;
        
        cursor.draw_cursor_line(cursor_line_draw_x, cursor_line_draw_y, gts.font_size as f32);

        draw_rectangle(draw_x, draw_y - gts.font_size as f32 + CURSOR_HEIGHT, cursor_width, gts.font_size as f32, CURSOR_COLOR);
    }

    // Determine visible lines
    let first_line = ((cam_top - start_y) / line_spacing).max(0.0) as usize;
    let last_line = ((cam_bottom - start_y) / line_spacing).min(text.len() as f32 - 1.0) as usize;

    let mut in_string = false;
    let mut in_block_comment = false;

    if !text.is_empty() {
        for line_index in first_line..=last_line {
            let line = &text[line_index];
            let y = start_y + line_index as f32 * line_spacing;
            let mut x = start_x + line_start_fix;

            // CRITICAL FIX: Replace tabs BEFORE processing
            let visual_line = line.replace("\t", TAB_PATTERN);
            
            let mut chars = visual_line.chars().peekable();
            while let Some(&c) = chars.peek() {
                let mut token = String::new();
                let color: Color;

                if in_block_comment {
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
                    while let Some(ch) = chars.next() {
                        token.push(ch);
                        if ch == '"' && !token.ends_with("\\\"") {
                            in_string = false;
                            break;
                        }
                    }
                    color = STRING_LITERAL_COLOR;
                } else {
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
                            while let Some(&ch) = chars.peek() {
                                if ch.is_whitespace() { break; }
                                token.push(chars.next().unwrap());
                            }
                            color = MACRO_COLOR;
                        }
                        c if c.is_whitespace() => {
                            while let Some(&ch) = chars.peek() {
                                if !ch.is_whitespace() { break; }
                                token.push(chars.next().unwrap());
                            }
                            color = IDENTIFIER_COLOR;
                        }
                        c if c.is_ascii_digit() => {
                            while let Some(&ch) = chars.peek() {
                                if !(ch.is_ascii_digit() || ch == '.' || ch == 'f' || ch == 'F' || ch == '-') { break; }
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
                                if !ch.is_alphanumeric() && ch != '_' { break; }
                                token.push(chars.next().unwrap());
                            }
                            let clean = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                            color = gts.calibrate_string_color(clean, &elk);
                        }
                    }
                }

                let width = measure_text(&token, Some(&gts.font), gts.font_size, 1.0).width;
                let (sx, sy) = camera.world_to_screen(x, y + text_y_offset);
                
                gts.color = color;
                gts.draw(&token, sx, sy);
                
                x += width;
            }
        }
    }

    // Sidebar
    let sidebar_width = start_x + line_start_fix - 5.0;
    draw_rectangle(0.0, 0.0, sidebar_width, screen_height(), COMPOSITE_TYPE_COLOR);
    draw_rectangle(0.0, 0.0, sidebar_width - 1.0, screen_height(), BACKGROUND_COLOR);

    // Line numbers
    gts.color = CURSOR_COLOR;
    for i in first_line..=last_line {
        let line_y_world = 1.1 * FILE_TEXT_X_MARGIN + FILE_LINE_NUMBER_Y_MARGIN + gts.font_size as f32 * i as f32 + text_y_offset;
        let screen_y = line_y_world - camera.offset_y;
        gts.draw(&i.to_string(), FILE_LINE_NUMBER_X_MARGIN, screen_y);
    }

    // Top bar
    let top_bar_height = MODE_Y_MARGIN + MODE_FONT_SIZE + text_y_offset;
    draw_rectangle(0.0, 0.0, screen_width(), top_bar_height + 1.0, COMPOSITE_TYPE_COLOR);
    draw_rectangle(0.0, 0.0, screen_width(), top_bar_height, BACKGROUND_COLOR);

    // Draw cursor position
    if !console.mode {
        let cursor_idx = format!("Ln {}, Col {}", cursor.xy.1, cursor.xy.0);
        gts.color = CONSOLE_TEXT_COLOR;
        let previous_size = gts.font_size;
        gts.font_size = 30; // Remains the same.
        gts.draw(&cursor_idx, MODE_Y_OFFSET, MODE_FONT_SIZE + MODE_Y_MARGIN + MODE_Y_OFFSET);
        gts.font_size = previous_size;
    }
}
