// Hide the scary console.
#![windows_subsystem = "windows"]

mod console;
mod text;
mod camera;
mod audio;
mod options;
mod win;

use std::vec;
use macroquad::prelude::*;

use crate::audio::editor_audio::EditorAudio;
use crate::camera::editor_camera::EditorCamera;
use crate::console::editor_console::{EditorConsole, console_message};
use crate::console::editor_file::{EditorFileSystem, draw_dir_contents, path_buffer_file_to_string, path_buffer_to_string};
use crate::options::editor_options::EditorOptions;
use crate::options::editor_pallete::{BACKGROUND_COLOR, COMPOSITE_TYPE_COLOR, FILE_COLOR, FOLDER_COLOR};
use crate::text::editor_cursor::{CURSOR_WORD_OFFSET, EditorCursor};
use crate::text::editor_input::record_keyboard_to_file_text;
use crate::text::editor_language_manager::{EditorLanguageKeywords ,load_keywords_for_extension};
use crate::text::editor_text::{CURRENT_FILE_TOP_BAR_OFFSET, MODE_FONT_SIZE, MODE_Y_MARGIN, MODE_Y_OFFSET, draw_file_text};
use crate::text::editor_text_stylizer::EditorGeneralTextStylizer;
use crate::win::editor_win_config::window_conf;

// TODO: Finish all the directives.
// TODO: Add Ctrl + z to undo last change.
// TODO: Add Ctrl + c to copy selected text.
// TODO: Add Ctrl + v to paste copied text.
// TODO: Add the palletes.
// TODO: Add more fonts.

// IDEA: Add selection mode
// IDEA: Add a list of user defined functions to make it easier to traverse files.
// IDEA: Add a list of user defined identifiers that will pop up as an autocomplete thing.
// IDEA: Add a cmd/terminal wrapper maybe, for compiling/executing code and git commands.
// IDEA: Add file markings in specific file indeces for faster traversal <:mark>.
// IDEA: Add file markings finder <:spot>, moves by one in each directive return, won't clear inside the console so the user can keep moving.
// IDEA: Add file markings finder <:spot N>, moves to the N-th marked spot inside the file. 

pub const VERSION: &str = "Muse-v01.05.02";

#[macroquad::main(window_conf())]
async fn main() {
    // Editor options
    let mut ops = EditorOptions::new();    
    // Editor camera
    let mut ec = EditorCamera::new();
    // File system
    let mut efs = EditorFileSystem::new();
    // Editor audio
    let audio = EditorAudio::new().await;
    // Editor general text stylizer
    let mut gts = EditorGeneralTextStylizer::new().await;
    // Editor Cursor
    let mut file_cursor = EditorCursor::new();
    // Console
    let mut console = EditorConsole::new();
    // Actual file text
    let mut file_text = vec![];
    // Language support based on file, default no higlighting
    let mut elk: EditorLanguageKeywords = load_keywords_for_extension("txt"); 

    let insert_word_w = measure_text("INSERT MODE", None, MODE_FONT_SIZE as u16, 1.0).width;
    let console_word_w = measure_text("CONSOLE MODE", None, MODE_FONT_SIZE as u16, 1.0).width;

    loop {
        clear_background(BACKGROUND_COLOR);

        draw_file_text(&mut file_text, &mut file_cursor, &mut gts, &console, &mut ec, &elk);
        if console.mode {
            console.draw(&gts);
        
            let is_cd = console.directive.starts_with(":cd ");
            let auto = draw_dir_contents(
                &efs.current_file,
                &efs.current_dir,
                &console.directive,
                &console,
                is_cd
            );
        
            if auto != "" {
                if is_cd {
                    console.directive = format!(":cd {}", auto);
                } else {
                    console.directive = auto;
                }
                console.cursor.x = console.directive.len();
            }
        }

        if !console.mode {
            record_keyboard_to_file_text(&mut file_cursor, &mut file_text, &audio, &mut console,  &mut gts, &mut efs, &mut ops, &mut elk);

            let mut fname = path_buffer_file_to_string(&efs.current_file);
            if efs.unsaved_changes {
                fname = format!("*{}", path_buffer_file_to_string(&efs.current_file));
            }

            draw_text("INSERT MODE", MODE_Y_OFFSET, MODE_FONT_SIZE + MODE_Y_MARGIN - 15.0, MODE_FONT_SIZE, COMPOSITE_TYPE_COLOR);
            draw_text(&path_buffer_to_string(&efs.current_dir), insert_word_w + 25.0, MODE_FONT_SIZE + MODE_Y_MARGIN - 15.0, MODE_FONT_SIZE, FOLDER_COLOR);
            draw_text(&fname, insert_word_w + CURRENT_FILE_TOP_BAR_OFFSET, MODE_FONT_SIZE + MODE_Y_MARGIN + 15.0, MODE_FONT_SIZE, FILE_COLOR);

            draw_text(&file_cursor.word, insert_word_w + CURRENT_FILE_TOP_BAR_OFFSET + CURSOR_WORD_OFFSET, MODE_FONT_SIZE + MODE_Y_MARGIN + 15.0, MODE_FONT_SIZE, BLUE);
        } else {
            console.record_keyboard_to_console_text(&audio, &mut efs, &mut file_text, &mut file_cursor, &mut ops, &mut elk);
            
            let mut fname = path_buffer_file_to_string(&efs.current_file);
            if efs.unsaved_changes {
                fname = format!("*{}", path_buffer_file_to_string(&efs.current_file));
            }
            
            draw_text("CONSOLE MODE", MODE_Y_OFFSET, MODE_FONT_SIZE + MODE_Y_MARGIN - 15.0, MODE_FONT_SIZE, COMPOSITE_TYPE_COLOR,);
            draw_text(&path_buffer_to_string(&efs.current_dir), console_word_w + 25.0, MODE_FONT_SIZE + MODE_Y_MARGIN - 15.0, MODE_FONT_SIZE, FOLDER_COLOR);
            draw_text(&fname, console_word_w + CURRENT_FILE_TOP_BAR_OFFSET, MODE_FONT_SIZE + MODE_Y_MARGIN + 15.0, MODE_FONT_SIZE, FILE_COLOR);
        }

        // Show message
        if console.showing_message && !console.message.is_empty() {
            console_message(&console.message, console.showing_manual);
        }

        // Nullify message
        if is_key_pressed(KeyCode::Escape) {
            console.showing_message = false;
            console.showing_manual = false;
            console.message.clear();
        }

        // if is_key_down(KeyCode::LeftAlt) {
        //     muse_draw_fps();
        // }
        
        muse_next_frame().await;
    }
}
