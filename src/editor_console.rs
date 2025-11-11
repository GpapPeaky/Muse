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

use crate::{editor_audio::EditorAudio, editor_text::*};

pub struct EditorConsole {
    pub mode: bool,
    pub directive: String,
    pub cursor: EditorConsoleCursor,
    pub message: String,
    pub showing_message: bool,
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
    fn record_special_console_keys(&mut self, audio: &EditorAudio, efs: &mut EditorFileSystem, text: &mut Vec<String>) {
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
            self.message = execute_directive(&mut self.directive, efs, text).clone();

            // Set console message toggle
            if self.message != "" {
                self.showing_message = true;
            }
        }
    }

    /// Record  heyboard input
    pub fn record_keyboard_to_console_text(&mut self, audio: &EditorAudio, efs: &mut EditorFileSystem, text: &mut Vec<String>) {
        self.record_special_console_keys(audio, efs, text);

        if let Some(c) = get_char_pressed() {
            if c.is_control() || c.is_ascii_control() {
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

/// Show a message error produced by the console
pub fn console_message(msg: &String) {
    let msg_font_size = 20;
    let text_width = measure_text(&msg, None, msg_font_size, 1.0).width;    
    
    let width = 60.0 + text_width;
    let height = 120.0;

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

    // Draw the message
    draw_text(&msg, screen_width() / 2.0 - text_width / 2.0,
            screen_height() / 2.0 - msg_font_size as f32 / 2.0,
            msg_font_size as f32,
            CONSOLE_FRAME_COLOR
    );

    let msg = "ESC to close";
    let text_esc_width = measure_text(&msg, None, msg_font_size, 1.0).width;

    draw_text(&msg, screen_width() / 2.0 - text_esc_width / 2.0,
            screen_height() / 2.0 - msg_font_size as f32 / 2.0 + msg_font_size as f32 + 10.0,
            msg_font_size as f32,
            CONSOLE_FRAME_COLOR
    );
}

pub fn console_manual(man_id: u8) {
      let width = screen_width() - 2.0;
      let height = screen_height() - 2.0;

      let text;      

      match man_id {
            // General manual
            0 => {
                  text = "
                    All directives have the ':' prefix, as the console will
                    handle input as a switch-to-file directive.
                    
                    Directives include:
                        File specific:
                                :l <N>      : Go to line N inside the file, if possible, else throw an error
                                :w          : Write the current open file                                    (C)
                                :i          : Current file info display
                                :r <f>      : Remove a file with name 'f'
                                :b <f>      : Change the name of the current open file to 'f'
                                :f <f>      : Go to the line where the first iteration of text 'f' exists
                                :c <f>      : Create a new file with name 'f'   
                    
                        Directory specific:
                                :cd         : Change directory                                         (C)
                                :od/o       : Open a directory, create process -> native file explorer (C)
                                :md <f>     : Create a new directory with name 'f'
                                :rd <f>     : Remove a directory with name 'f' with all its contents
                                :bd <f>     : Change the name of the current open directory to 'f'
                    
                        Conf: <saved in cal.conf file>
                                :epa  <p>   : Change to pallete of name 'p'
                                :efn  <p>   : Change to a font of name 'p'
                                :efs <N>    : Change font size to N
                                :eau        : Audio on/off switch
                                :eav <N>    : Set editor audio volume to N
                                :esi        : Smart identation on/off switch
                                :efl        : Editor fullscreen switch
                                :ehi        : Editor highlighting toggle
                                :ewt        : Editor cursor width toggle
                    
                        Other:
                                :e/q        : Exit, close editor                                            (C)
                                :egman      : Editor general manual (All manuals are displayed)
                                :efman      : Editor file manual    (Display file directives info)
                                :edman      : Editor directory manual  (Display directory directives info)
                                :ecman      : Editor config manual  (Display editor config directives info)
                                :eoman      : Editor others manual  (Display editor other directives info)
                                :ever       : Editor version
                                :eck        : Editor clock (current time and time opened)
                                :egam <N>   : Editor gamble, display a number from 0 to N
                ";
            }
                    
            // File manual
            1 => {
                text = "
                    File specific directives:
                        :l <N>      : Go to line N inside the file, if possible, else throw an error
                        :w          : Write the current open file                                    (C)
                        :i          : Current file info display
                        :r <f>      : Remove a file with name 'f'
                        :b <f>      : Change the name of the current open file to 'f'
                        :f <f>      : Go to the line where the first iteration of text 'f' exists
                        :c <f>      : Create a new file with name 'f'
                ";
            }      
            
            // Directory manual            
            2 => {
                text = "
                    Directory specific:
                        :cd         : Change directory                                         (C)
                        :od/o       : Open a directory, create process -> native file explorer (C)
                        :md <f>     : Create a new directory with name 'f'
                        :rd <f>     : Remove a directory with name 'f' with all its contents
                        :bd <f>     : Change the name of the current open directory to 'f'
                "
            }
            
            // Options manual
            3 => {
                  text = "
                    Conf directives: <saved in cal.conf file>
                        :epa  <p>   : Change to pallete of name 'p'
                        :efn  <p>   : Change to a font of name 'p'
                        :efs <N>    : Change font size to N
                        :eau        : Audio on/off switch
                        :eav <N>    : Set editor audio volume to N
                        :esi        : Smart identation on/off switch
                        :efl        : Editor fullscreen switch
                        :ehi        : Editor highlighting toggle
                        :ewt        : Editor cursor width toggle
                ";
            }

            // Other manual
            4 => {
                text = "
                    Other directives:
                        :e/q        : Exit, close editor                                            (C)
                        :egman      : Editor general manual (All manuals are displayed)
                        :efman      : Editor file manual    (Display file directives info)
                        :edman      : Editor directory manual  (Display directory directives info)
                        :ecman      : Editor config manual  (Display editor config directives info)
                        :eoman      : Editor others manual  (Display editor other directives info)
                        :ever       : Editor version
                        :eck        : Editor clock (current time and time opened)
                        :egam <N>   : Editor gamble, display a number from 0 to N
                ";
            }

            _ => {
                text = "Manual requested not found.";
            }
      }
      
      draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            WHITE
      );

      draw_rectangle(
        1.0,
        1.0,
        width, 
        height,
        BLACK
    );

    draw_text(text, 50.0, 50.0, 20.0, WHITE);
      
}

