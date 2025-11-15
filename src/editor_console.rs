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

#[path = "editor_pallete.rs"]
mod editor_pallete;
use editor_pallete::*;

use crate::{editor_audio::EditorAudio, editor_cursor::EditorCursor, editor_text::*};

pub struct EditorConsole {
    pub mode: bool,
    pub directive: String,
    pub cursor: EditorConsoleCursor,
    pub message: String,
    pub showing_message: bool,
    pub showing_manual: bool,
}

const CONSOLE_WIDTH: f32 = 400.0;
pub const CONSOLE_MARGINS: f32 = 15.0;

impl EditorConsole {
    /// Console constructor
    pub fn new() -> EditorConsole {
        EditorConsole { mode: false,
            directive: String::new(),
            cursor: EditorConsoleCursor::new(),
            message: String::new(),
            showing_message: false,
            showing_manual: false
        }
    }

    /// Console will be drawn to the right of the screen
    pub fn draw(&self) {
        // Console background
        draw_rectangle(screen_width() - CONSOLE_WIDTH,
            0.0,
            CONSOLE_WIDTH,
            screen_height(),
            CONSOLE_FRAME_COLOR
        );

        // Console foreground
        draw_rectangle(screen_width() - CONSOLE_WIDTH + 1.5,
            0.0,
            CONSOLE_WIDTH,
            screen_height(),
            CONSOLE_CONTAINER_COLOR
        );

        draw_line(screen_width() - CONSOLE_WIDTH,
            CONSOLE_MARGINS + 25.0,
            screen_width(),
            CONSOLE_MARGINS + 25.0,
            1.0,
            CONSOLE_FRAME_COLOR
        );

        let directive_len: f32 = measure_text(&self.directive, None, 30, 1.0).width;

        // Console cursor
        draw_line(screen_width() - CONSOLE_WIDTH + CONSOLE_MARGINS + directive_len
            ,CONSOLE_MARGINS
            ,screen_width() - CONSOLE_WIDTH + CONSOLE_MARGINS + directive_len,
            CONSOLE_MARGINS + 15.0,
            2.0,
            CONSOLE_CURSOR_COLOR);

        draw_text(&self.directive,
            screen_width() - CONSOLE_WIDTH + CONSOLE_MARGINS - 5.0,
            CONSOLE_MARGINS + 15.0,
            30.0,
            CONSOLE_TEXT_COLOR
        );
    }

    /// Special input, backspace and enter
    fn record_special_console_keys(&mut self, audio: &EditorAudio, efs: &mut EditorFileSystem, text: &mut Vec<String>, cursor: &mut EditorCursor) {
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
            let message_and_manual_toggle = execute_directive(&mut self.directive, efs, text, cursor).clone();

            // Update for rendering.
            self.message = message_and_manual_toggle.0;
            self.showing_manual = message_and_manual_toggle.1;

            // Set console message toggle
            if self.message != "" {
                self.showing_message = true;
            }
        }
    }

    /// Record  heyboard input
    pub fn record_keyboard_to_console_text(&mut self, audio: &EditorAudio, efs: &mut EditorFileSystem, text: &mut Vec<String>, cursor: &mut EditorCursor) {
        self.record_special_console_keys(audio, efs, text, cursor);

        // Disable special characters from the console.
        if let Some(c) = get_char_pressed() {
            if !c.is_ascii_alphanumeric() && c != '_' && c != '-' && c != ' ' && c != '.' && c != '/' && c != '\\' && c != ':' && c != '<' && c != '>' {
                return;
            }

            match c {
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

/// Draws multi-line text
pub fn draw_multiline_text_centered(text: &str, font_size: u16, color: Color, start_y: f32) {
    let line_height = font_size as f32 + 5.0; // 5px padding between lines
    let mut y = start_y;

    for line in text.lines() {
        draw_text(
            line,
            5.0,
            y,
            font_size as f32,
            color,
        );

        y += line_height;
    }
}

/// Show a message error produced by the console
pub fn console_message(msg: &String, is_manual: bool) {
    let msg_font_size = 18;
    let text_width = measure_text(&msg, None, msg_font_size, 1.0).width;    
    
    let width = 60.0 + text_width;
    let height = 120.0;

    if is_manual {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            CONSOLE_FRAME_COLOR
        );

        draw_rectangle(
            1.0,
            1.0,
            screen_width() - 2.0,
            screen_height() - 2.0,
            CONSOLE_CONTAINER_COLOR
        );
    } else {
        draw_rectangle(
            screen_width() / 2.0 - width / 2.0 - 1.0,
            screen_height() / 2.0 - height / 2.0 - 1.0,
            width + 1.0,
            height + 1.0,
            CONSOLE_FRAME_COLOR
        );

        draw_rectangle(
            screen_width() / 2.0 - width / 2.0,
            screen_height() / 2.0 - height / 2.0,
            width - 1.0,
            height - 1.0,
            CONSOLE_CONTAINER_COLOR
        );
    }

    // Draw the message
    if is_manual {
        let start_y = 5.0;
        draw_multiline_text_centered(msg, msg_font_size, CONSOLE_FRAME_COLOR, start_y);
    } else {
        draw_text(
            msg,
            screen_width() / 2.0 - text_width / 2.0,
            screen_height() / 2.0,
            msg_font_size as f32,
            CONSOLE_FRAME_COLOR
        );
    }

    // Draw ESC tip
    let esc_msg = "ESC to close";
    let esc_width = measure_text(esc_msg, None, msg_font_size, 1.0).width;

    draw_text(
        esc_msg,
        screen_width() / 2.0 - esc_width / 2.0,
        screen_height() / 2.0 + height / 2.0 - 20.0,
        msg_font_size as f32,
        CONSOLE_FRAME_COLOR
    );
}

/// Choose a console manual.
pub fn console_manual(man_id: u8) -> String {
      let text;      

      match man_id {
            // General manual
            0 => {
                  text =
                  "
                File specific directives:
                :l <N>      : Go to line N inside the file, if possible, else throw an error
                :w          : Write the current open file                                   
                :i          : Current file info display
                :r <f>      : Remove a file with name 'f'
                :b <f>      : Change the name of the current open file to 'f'
                :f <f>      : Go to the line where the first iteration of text 'f' exists
                :c <f>      : Create a new file with name 'f'   
                    
                Directory specific directives:
                :cd         : Change directory                                        
                :od/o       : Open a directory, create process -> native file explorer
                :md <f>     : Create a new directory with name 'f'
                :rd <f>     : Remove a directory with name 'f' with all its contents
                :bd <f>     : Change the name of the current open directory to 'f'
                    
                Option specific directives:
                :epa  <p>   : Change to pallete of name 'p'
                :efn  <p>   : Change to a font of name 'p'
                :efs <N>    : Change font size to N
                :eau        : Audio on/off switch
                :eav <N>    : Set editor audio volume to N
                :esi        : Smart identation on/off switch
                :efl        : Editor fullscreen switch
                :ehi        : Editor highlighting toggle
                    
                Other directives:
                :e/q                : Exit, close editor                                           
                :egman              : Editor general manual (All manuals are displayed)
                :efman              : Editor file manual    (Display file directives info)
                :edman              : Editor directory manual  (Display directory directives info)
                :ecman              : Editor config manual  (Display editor config directives info)
                :eoman              : Editor others manual  (Display editor other directives info)
                :ectrl              : Editor controls manual (Display editor controls info)
                :ever               : Editor version
                :eck                : Editor clock (current time and time opened)
                :egam/rand/roll <N> : Editor gamble, display a number from 0 to N 
                ".to_string();
            }
                    
            // File manual
            1 => {
                text = "
                    File specific directives:
                        :l <N>      : Go to line N inside the file, if possible, else throw an error
                        :w          : Write the current open file                                   
                        :i          : Current file info display
                        :r <f>      : Remove a file with name 'f'
                        :b <f>      : Change the name of the current open file to 'f'
                        :f <f>      : Go to the line where the first iteration of text 'f' exists
                        :c <f>      : Create a new file with name 'f'
                ".to_string();
            }      
            
            // Directory manual            
            2 => {
                text = "
                    Directory specific directives:
                        :cd         : Change directory                                        
                        :od/o       : Open a directory, create process -> native file explorer
                        :md <f>     : Create a new directory with name 'f'
                        :rd <f>     : Remove a directory with name 'f' with all its contents
                        :bd <f>     : Change the name of the current open directory to 'f'
                ".to_string();
            }
            
            // Options manual
            3 => {
                  text = "
                    Options directives:
                        :epa  <p>   : Change to pallete of name 'p'
                        :efn  <p>   : Change to a font of name 'p'
                        :efs <N>    : Change font size to N
                        :eau        : Audio on/off switch
                        :eav <N>    : Set editor audio volume to N
                        :esi        : Smart identation on/off switch
                        :efl        : Editor fullscreen switch
                        :ehi        : Editor highlighting toggle
                ".to_string();
            }

            // Other manual
            4 => {
                text = "
                    Other directives:
                        :e/q                : Exit, close editor                                           
                        :egman/man          : Editor general manual (All manuals are displayed)            
                        :efman              : Editor file manual    (Display file directives info)         
                        :edman              : Editor directory manual  (Display directory directives info) 
                        :ecman              : Editor config manual  (Display editor config directives info)
                        :eoman              : Editor others manual  (Display editor other directives info) 
                        :ectrl              : Editor controls manual (Display editor controls info)
                        :ever               : Editor version                                               
                        :eck                : Editor clock (current time and time opened)
                        :egam/rand/roll <N> : Editor gamble, display a number from 0 to N 
                ".to_string();
            }

            5 => {
               text= "
                  Infile controls: 
                     ArrowKeys: Move the cursor index by one vertically/horizontally.
                     LCtrl + ArrowKeys: Move the cursor index to the next non whitespace character
                                        horizontally, or by 5 vertically.
                     LCtrl + LShift + ArrowKeys: Smoothly slide the cursor vertically. 
               ".to_string();
            }

            _ => {
                text = "".to_string();
            }
      }

    return text;
}
