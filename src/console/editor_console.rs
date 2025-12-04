// Console module, see editor_directives.rs 
// for more info.

use macroquad::prelude::*;

use crate::audio::editor_audio::*;
use crate::options::editor_options::EditorOptions;
use crate::options::editor_pallete::{
    CONSOLE_CONTAINER_COLOR,
    CONSOLE_CURSOR_COLOR,
    CONSOLE_FRAME_COLOR
};
use crate::console::editor_console_cursor::*;
use crate::console::editor_file::*;
use crate::text::editor_cursor::*;
use crate::text::editor_text_stylizer::*;
use crate::console::editor_directives::*;
use crate::text::editor_language_manager::EditorLanguageKeywords;

pub const CONSOLE_INITIAL_WIDTH: f32 = 250.0;
pub const CONSOLE_MARGINS: f32 = 15.0;

pub const CONSOLE_RESIZE_STEP: f32 = 30.0;

pub struct EditorConsole {
    pub mode: bool,
    pub directive: String,
    pub cursor: EditorConsoleCursor,
    pub message: String,
    pub showing_message: bool,
    pub showing_manual: bool,
    pub width: f32,
    pub target_w: f32,
    pub vel_w: f32,
}

impl EditorConsole {
    /// Console constructor
    pub fn new() -> EditorConsole {
        EditorConsole { 
            mode: false,
            directive: String::new(),
            cursor: EditorConsoleCursor::new(),
            message: String::new(),
            showing_message: false,
            showing_manual: false,
            width: CONSOLE_INITIAL_WIDTH,
            target_w: CONSOLE_INITIAL_WIDTH,
            vel_w: 1.0,
        }
    }

    /// Animate width through interpolating
    pub fn animate_width(&mut self,) {
        let stiffness = 0.35;
        let damping = 0.3;
    
        let dx = self.target_w - self.width;
    
        self.vel_w += dx * stiffness;
        self.vel_w *= damping;
    
        self.width += self.vel_w;
    }

    /// Resize console and interpolate
    pub fn resize_console(
        &mut self,
        leftorright: bool
    ) {
        // Left
        if leftorright {
            self.target_w += CONSOLE_RESIZE_STEP;
        } else { // Right
            self.target_w -= CONSOLE_RESIZE_STEP;
        }
    }

    /// Console will be drawn to the right of the screen
    pub fn draw(
        &mut self,
        _gts: &EditorGeneralTextStylizer
    ) {
        // Console background
        draw_rectangle(screen_width() - self.width,
            0.0,
            self.width,
            screen_height(),
            CONSOLE_FRAME_COLOR
        );

        // Console foreground
        draw_rectangle(screen_width() - self.width + 1.5,
            0.0,
            self.width,
            screen_height(),
            CONSOLE_CONTAINER_COLOR
        );

        draw_line(screen_width() - self.width,
            CONSOLE_MARGINS + 25.0,
            screen_width(),
            CONSOLE_MARGINS + 25.0,
            1.0,
            CONSOLE_FRAME_COLOR
        );

        let cursor_idx = char_to_byte(&self.directive, self.cursor.x);
        let cursor_text = &self.directive[..cursor_idx];
        let cursor_w = measure_text(cursor_text, None, 30, 1.0).width;

        // Interpolate
        self.cursor.animate_to(screen_width() - self.width + CONSOLE_MARGINS + cursor_w - 5.0);

        // Console cursor
        draw_line(
            self.cursor.anim_x,
            CONSOLE_MARGINS - 5.0,
            self.cursor.anim_x,
            CONSOLE_MARGINS + 20.0,
            2.0,
            CONSOLE_CURSOR_COLOR
        );

        // Draw the directive written
        _gts.draw(
            &self.directive,
            screen_width() - self.width + CONSOLE_MARGINS - 5.0,
            CONSOLE_MARGINS + 15.0,
        );
    }

    fn lshift_shortcuts(
        &mut self,
        audio: &EditorAudio
    ) -> bool {
        // Left, resize console
        if is_key_down(KeyCode::Left) {
            self.resize_console(true);
            audio.play_nav();   

            return true;
        }   

        // Right, resize console
        if is_key_down(KeyCode::Right) {
            self.resize_console(false);
            audio.play_nav();   

            return true;
        }

        false
    }

    /// Special input, backspace and enter
    fn record_special_console_keys(
        &mut self,
        audio: &EditorAudio,
        efs: &mut EditorFileSystem,
        text: &mut Vec<String>,
        cursor: &mut EditorCursor,
        ops: &mut EditorOptions,
        elk: &mut EditorLanguageKeywords,
    ) {
        if cursor.is_combo_active(KeyCode::Backspace, None) {
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
        }

        // Resizing
        if cursor.is_combo_active(KeyCode::LeftShift, None) {
            self.lshift_shortcuts(audio);
        }

        if is_key_pressed(KeyCode::Enter) {
            // execute whatever is inside the directive string
            // check the directives' source
            let message_and_manual_toggle = execute_directive(&mut self.directive, efs, text, cursor, ops, elk).clone();

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
    pub fn record_keyboard_to_console_text(
        &mut self,
        audio: &EditorAudio,
        efs: &mut EditorFileSystem,
        text: &mut Vec<String>,
        cursor: &mut EditorCursor,
        ops: &mut EditorOptions,
        elk: &mut EditorLanguageKeywords,
    ) {
        self.record_special_console_keys(audio, efs, text, cursor, ops, elk);

        // Disable special characters from the console.
        if let Some(c) = get_char_pressed() {
            if c.is_control() {
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

        console_text_navigation(&mut self.cursor.x, &mut self.directive, audio);
    }
}

/// Draws multi-line text
pub fn draw_multiline_text_centered(
    text: &str,
    font_size: u16,
    color: Color,
    start_y: f32
) {
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
pub fn console_message(
    msg: &String, 
    is_manual: bool
) {
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
        let start_y = 15.0;
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
                :t <c>      : Execute a command 'c' terminal
                    
                Directory specific directives:
                :cd         : Change directory                                        
                :od/o       : Open a directory, create process -> native file explorer
                :md <f>     : Create a new directory with name 'f'
                :rd <f>     : Remove a directory with name 'f' with all its contents
                :bd <f>     : Change the name of the current open directory to 'f'
                                    
                Configuration directives:
                :epa <p>    : Change to pallete of name 'p'
                :efn <p>    : Change to a font of name 'p'
                :esm        : Smart identation on/off switch
                :eau        : Audio on/off switch
                :efl        : Editor fullsreen on/off switch
                :ehi        : Editor text highlighting on/off switch
                :e/q                : Exit, close editor                                           
                    
                Other directives:
                :egman              : Editor general manual (All manuals are displayed)
                :efman              : Editor file manual    (Display file directives info)
                :edman              : Editor directory manual  (Display directory directives info)
                :eoman              : Editor others manual  (Display editor other directives info)
                :ecman              : Editor config manual  (Display editor config directives info)
                :ectrl              : Editor controls manual (Display editor controls info)
                :ever               : Editor version
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
                        :t <c>      : Execute a command 'c' terminal
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
                ".to_string();
            }
            
            // Options manual
            3 => {
                  text = "
                    Configuration directives:
                        :epa <p>    : Change to pallete of name 'p'
                        :efn <p>    : Change to a font of name 'p'
                        :eau        : Audio on/off switch
                        :esm        : Smart identation on/off switch
                        :efl        : Editor fullscreen on/off switch
                        :ehi        : Edito highlighting on/off switch
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
                     
                     // TODO: Add shortcuts
               ".to_string();
            }

            _ => {
                text = "".to_string();
            }
      }

    return text;
}
