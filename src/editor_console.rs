// Console module, see editor_directives.rs 
// for more info.

use macroquad::prelude::*;

#[path = "editor_file.rs"]
pub mod editor_file;
use editor_file::*;

#[path = "editor_directives.rs"]
mod editor_directives;
use editor_directives::*;

#[path = "editor_console_cursor.rs"]
mod editor_console_cursor;
use editor_console_cursor::*;

use crate::{editor_audio::EditorAudio, editor_text::*};

pub struct EditorConsole {
    pub mode: bool,
    pub directive: String,
    pub cursor: EditorConsoleCursor
}

const CONSOLE_WIDTH: f32 = 255.0;
pub const CONSOLE_MARGINS: f32 = 15.0;

impl EditorConsole {
    /// Console constructor
    pub fn new() -> EditorConsole {
        EditorConsole { mode: false,
            directive: String::new(),
            cursor: EditorConsoleCursor::new()
        }
    }

    /// Console will be drawn to the right of the screen
    pub fn draw(&self) {
        // Console background
        draw_rectangle(screen_width() - CONSOLE_WIDTH,
            0.0,
            CONSOLE_WIDTH,
            screen_height(),
            COMPOSITE_TYPE_COLOR
        );

        // Console foreground
        draw_rectangle(screen_width() - CONSOLE_WIDTH + 1.5,
            0.0,
            CONSOLE_WIDTH,
            screen_height(),
            BACKGROUND_COLOR
        );

        draw_line(screen_width() - CONSOLE_WIDTH,
            CONSOLE_MARGINS + 25.0,
            screen_width(),
            CONSOLE_MARGINS + 25.0,
            1.0,
            COMPOSITE_TYPE_COLOR
        );

        let directive_len: f32 = measure_text(&self.directive, None, 30, 1.0).width;

        draw_line(screen_width() - CONSOLE_WIDTH + CONSOLE_MARGINS + directive_len
            ,CONSOLE_MARGINS
            ,screen_width() - CONSOLE_WIDTH + CONSOLE_MARGINS + directive_len,
            CONSOLE_MARGINS + 15.0,
            2.0,
            STORAGE_CLASS_COLOR);

        draw_text(&self.directive,
            screen_width() - CONSOLE_WIDTH + CONSOLE_MARGINS - 5.0,
            CONSOLE_MARGINS + 15.0,
            30.0,
            STORAGE_CLASS_COLOR
        );
    }

    /// Special input, backspace and enter
    fn record_special_console_keys(&mut self, audio: &EditorAudio, efs: &mut EditorFileSystem) {
        if is_key_pressed(KeyCode::Backspace) {
            if self.cursor.x > 0 && !self.directive.is_empty() {
                let mut byte_idx = char_to_byte(&self.directive, self.cursor.x - 1);
            
                // Clamp if it's at the end
                if byte_idx >= self.directive.len() {
                    byte_idx = self.directive
                        .char_indices()
                        .last()
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                }
            
                self.directive.remove(byte_idx);
                self.cursor.x -= 1;
                audio.play_delete();
            }

            return;
        }

        if is_key_down(KeyCode::LeftControl) {
            if is_key_pressed(KeyCode::GraveAccent) {
                self.mode = false;
            }

            console_text_special_navigation(&mut self.cursor.x, &mut self.directive, audio);
        } else {
            console_text_navigation(&mut self.cursor.x, &mut self.directive, audio);
        }

        if is_key_pressed(KeyCode::Enter) {
            // execute whatever is inside the directive string
            // check the directives' source
            execute_directive(&mut self.directive, efs);
        }
    }

    /// Record  heyboard input
    pub fn record_keyboard_to_console_text(&mut self, audio: &EditorAudio, efs: &mut EditorFileSystem) {
        self.record_special_console_keys(audio, efs);

        if let Some(c) = get_char_pressed() {
            match c {
                '\u{8}' | '\r' | '\n' | '\t' => {
                    return;
                }

                _ => {
                    if c != ' ' { 
                        audio.play_insert();
                    } else {
                        audio.play_space();
                    }

                    let byte_idx = char_to_byte(&self.directive, self.cursor.x);
                    self.directive.insert(byte_idx, c);
                    self.cursor.x += 1;
                }
            }
        }
    }
}
