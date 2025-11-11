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
//              :w          : Write the current open file                                    (C)
//              :i          : Current file info display
//              :r <f>      : Remove a file with name 'f'
//              :b <f>      : Change the name of the current open file to 'f'
//              :f <f>      : Go to the line where the first iteration of text 'f' exists
//              :c <f>      : Create a new file with name 'f'   
//
//      Directory specific:
//              :cd         : Change directory                                         (C)
//              :od/o       : Open a directory, create process -> native file explorer (C)
//              :md <f>     : Create a new directory with name 'f'
//              :rd <f>     : Remove a directory with name 'f' with all its contents
//              :bd <f>     : Change the name of the current open directory to 'f'
//
//      Conf: <saved in cal.conf file>
//              :epa <p>    : Change to pallete of name 'p'
//              :efn <p>    : Change to a font of name 'p'
//              :efs <N>    : Change font size to N
//              :eau        : Audio on/off switch
//              :eav <N>    : Set editor audio volume to N
//              :esi        : Smart identation on/off switch
//              :efl        : Editor fullscreen switch
//              :ehi        : Editor highlighting toggle
//              :ewt        : Editor cursor width toggle
//
//      Other:
//              :e/q        : Exit, close editor                                            (C)
//              :egman/man  : Editor general manual (All manuals are displayed)             (C)
//              :efman      : Editor file manual    (Display file directives info)          (C)
//              :edman      : Editor directory manual  (Display directory directives info)  (C)
//              :ecman      : Editor config manual  (Display editor config directives info) (C)
//              :eoman      : Editor others manual  (Display editor other directives info)  (C)
//              :ever       : Editor version                                                (C)
//              :eck        : Editor clock (current time and time opened)
//              :egam <N>   : Editor gamble, display a number from 0 to N
//
// When the console is faced with a directive without a ':' prefix
// it will view it as a switch-to-file command and will try to switch 
// to a file with that name if found, same with directorys.
// The console, as long as you are typing, will display files with names close to it.
// Pressing TAB will select the first seen file closest to the name given and autocomplete it
// in the console.

use crate::editor_console::{console_manual, editor_file::*};

/// Check if there is a ':', trim it, match it to a directive and execute it
/// else we will see it as switch-to-file operation
/// returns a message if there is an error OR a manual to show
/// as well as boolean to delcare if it's a manual
pub fn execute_directive(directive: &mut String, efs: &mut EditorFileSystem, text: &mut Vec<String>) -> (String, bool) {
    if directive.starts_with(':') {
        let directive_command = directive.trim_start_matches(':').trim();
        let mut tokens = directive_command.split_whitespace();
        let command = tokens.next().unwrap_or("");
        let parameter = tokens.next();

        match command {
            "od" | "o" | "O" | "Od" | "oD" | "OD" => efs.open_file_explorer(),

            "c" | "C" => {
                if let Some(param) = parameter {
                    let r = efs.create_file(param);

                    if !r {
                        return ("FileNameUsed".to_string(), false);
                    }

                    efs.change_current_file(param.to_string());
                    *text = efs.load_current_file().unwrap_or_default();
                } else {
                    return ("NoFileNameProvided <:c>".to_string(), false);
                }
            }

            "cd" | "CD" | "Cd" | "cD" => {
                if let Some(param) = parameter {
                    efs.change_current_directory(param.to_string());

                    if let Some(current_file) = &efs.current_file {
                        let path = efs.current_dir.clone().unwrap_or_default().join(current_file);
                        if path.exists() {
                            *text = efs.load_current_file().unwrap_or_default();
                        } else {
                            return ("FileNotFound".to_string(), false);
                        }
                    }
                } else {
                    return ("NoDirectoryNameProvided <:cd>".to_string(), false);
                }
            }

            "w" | "W" => {
                let _ = efs.write_current_file(text);
            }

            "e" | "q" | "E" | "Q" => std::process::exit(0),

            // Manuals
            "egman" | "man" => return (console_manual(0), true),
            "efman"         => return (console_manual(1), true),
            "edman"         => return (console_manual(2), true),
            "ecman"         => return (console_manual(3), true),
            "eoman"         => return (console_manual(4), true),

            "ever" => return ("Muse v1.2.0".to_string(), false),

            _ => return ("UnknownDirective".to_string(), false),
        }
    } else {
        // File switch
        if efs.change_current_file(directive.to_string()) {
            text.clear();
            *text = efs.load_current_file().unwrap_or_default();
        } else {
            text.clear();
            efs.current_file = None;
            return ("FileNotFound".to_string(), false);
        }
    }

    *directive = String::new();

    return ("".to_string(), false);
}
