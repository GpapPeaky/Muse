// The console will be allowed to execute directives
// that enables the user to switch from directory to directory
// write files, remove files, visit a line in a file
// , change pallete and create files.
//
// The user can go from console, to insert mode and (vize versa)
// by pressing 'CTRL +`'.
//
// All directives have the ':' prefix, as the console will
// handle input as a switch-to-file directive.
//
// Directives include:
//      File specific:
//              :l <N>      : Go to line N inside the file, if possible, else throw an error
//              :w          : Write the current open file
//              :i          : Current file info display
//              :r <f>      : Remove a file with name 'f'
//              :b <f>      : Change the name of the current open file to 'f'
//              :f <f>      : Go to the line where the first iteration of text 'f' exists
//              :c <f>      : Create a new file with name 'f'   
//
//      Directory specific:
//              :cd         : Change directory                                         (C)
//              :od         : Open a directory, create process -> native file explorer (C)
//              :md <f>     : Create a new directory with name 'f'
//              :rd <f>     : Remove a directory with name 'f' with all its contents
//              :bd <f>     : Change the name of the current open directory to 'f'
//              :ld         : List all files in the directory
//              :td         : Show all contents of the directory recursively
//
//      Conf: <saved in cal.conf file>
//              :epa  <p>   : Change to pallete of name 'p'
//              :efn  <p>   : Change to a font of name 'p'
//              :efs <N>    : Change font size to N
//              :eau        : Audio on/off switch
//              :eav <N>    : Set editor audio volume to N
//              :esi        : Smart identation on/off switch
//              :efl        : Editor fullscreen switch
//              :ehi        : Editor highlighting toggle
//              :ewt        : Editor cursor width toggle
//
//      Other:
//              :e          : Exit, close editor                                            (C)
//              :egman      : Editor general manual (All manuals are displayed)
//              :efman      : Editor file manual    (Display file directives info)
//              :edman      : Editor directory manual  (Display directory directives info)
//              :ecman      : Editor config manual  (Display editor config directives info)
//              :eoman      : Editor others manual  (Display editor other directives info)
//              :ever       : Editor version
//              :eck        : Editor clock (current time and time opened)
//              :egam <N>   : Editor gamble, display a number from 0 to N
//
// When the console is faced with a directive without a ':' prefix
// it will view it as a switch-to-file command and will try to switch 
// to a file with that name if found, same with directorys.
// The console, as long as you are typing, will display files with names close to it.
// Pressing TAB will select the first seen file closest to the name given and autocomplete it
// in the console.

use crate::editor_console::editor_file::*;

use std::process::exit;

/// Check if there is a ':', trim it, match it to a directive and execute it
/// else we will see it as switch-to-file operation
pub fn execute_directive(directive: &mut String, efs: &mut EditorFileSystem) {
    // Check if there is a ':'
    // Trim it, match it to a directive
    // and execute it

    if directive.starts_with(':') {
        let directive_command = directive.trim_matches(':');

        let mut tokens = directive_command.split_whitespace();
        let command = tokens.next().unwrap_or("");
        let parameter = tokens.next(); // Option<&str>

        match command {
            "od" => {
                efs.open_file_explorer();
            }

            "cd" => {
                if let Some(parameter) = parameter {
                    efs.change_current_directory(parameter.to_string());
                } else {
                    println!("No directory provided for :cd");
                }
            }

            "e" => exit(0),

            "ewt" => {
                // TODO: Add an options object
                // CURSOR_LINE_TO_WIDTH = !CURSOR_LINE_TO_WIDTH;
            }

            "efl" => {

            }

            _ => {
                // TODO: Needs a timer
                // draw_text("INVALID DIRECTIVE", x, y, font_size, color)
                println!("Invalid directive {}", command);
            }

        }
    } else {
        // Simple file switch
        if !efs.change_current_file(directive.to_string()) {
            // TODO: Needs a timer
            // draw_text("FILE {diretive.to_string()} NOT FOUND", x, y, font_size, color)
        }
    }

    // Remove dat shii
    *directive = "".to_string();
}
