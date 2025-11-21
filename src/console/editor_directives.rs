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
//              :w          : Write the current open file                                    (C)
//              :r <f>      : Remove a file with name 'f'                                    (C)
//              :c <f>      : Create a new file with name 'f'                                (C)
//              :l <N>      : Go to line N inside the file, if possible, else throw an error (C)
//              :b <f>      : Change the name of the current open file to 'f'                (C)
//              :i          : Current file info display
//              :f <f>      : Go to the line where the first iteration of text 'f' exists
//
//      Directory specific:
//              :cd         : Change directory                                         (C)
//              :od/o       : Open a directory, create process -> native file explorer (C)
//              :md <f>     : Create a new directory with name 'f'                     (C)
//              :rd <f>     : Remove a directory with name 'f' with all its contents   (C)
//
//      Conf: <saved in cal.conf file>
//              :epa <p>    : Change to pallete of name 'p'
//              :efn <p>    : Change to a font of name 'p'
//              :eau        : Audio on/off switch
//              :eav <N>    : Set editor audio volume to N
//
//      Other:
//              :e/q                : Exit, close editor                                            (C)
//              :egman/man          : Editor general manual (All manuals are displayed)             (C)
//              :efman              : Editor file manual    (Display file directives info)          (C)
//              :edman              : Editor directory manual  (Display directory directives info)  (C)
//              :ecman              : Editor config manual  (Display editor config directives info) (C)
//              :eoman              : Editor others manual  (Display editor other directives info)  (C)
//              :ectrl              : Editor controls manual (Display editor controls info)         (C)
//              :ever               : Editor version                                                (C)
//              :egam/rand/roll <N> : Editor gamble, display a number from 0 to N                   (C)
//
// When the console is faced with a directive without a ':' prefix
// it will view it as a switch-to-file command and will try to switch 
// to a file with that name if found, same with directorys.
// The console, as long as you are typing, will display files with names close to it.
// Pressing TAB will select the first seen file closest to the name given and autocomplete it
// in the console.

use std::path::Path;
use std::str::FromStr;

use macroquad::prelude::rand;

use crate::options::editor_options::*;
use crate::console::editor_console::*;
use crate::console::editor_file::*;
use crate::text::editor_cursor::*;
use crate::VERSION;
use crate::text::editor_language_manager::EditorLanguageKeywords;
use crate::text::editor_language_manager::load_keywords_for_extension;
use crate::text::editor_text::find_word_in_text;

/// Check if there is a ':', trim it, match it to a directive and execute it
/// else we will see it as switch-to-file operation
/// returns a message if there is an error OR a manual to show
/// as well as boolean to delcare if it's a manual
pub fn execute_directive(
    directive: &mut String,
    efs: &mut EditorFileSystem, 
    text: &mut Vec<String>, 
    cursor: &mut EditorCursor,
    ops: &mut EditorOptions,
    elk: &mut EditorLanguageKeywords
) -> (String, bool) {
    if directive.starts_with(':') {
        let directive_command = directive.trim_start_matches(':').trim();
        let mut tokens = directive_command.split_whitespace();
        let command = tokens.next().unwrap_or("");
        let parameter = tokens.next();

        match command {
            "od" | "o" | "O" | "Od" | "oD" | "OD" => efs.open_file_explorer(),

            "B" | "b" => {
                if let Some(param) = parameter {
                    let r = efs.baptize_file(param);

                    if !r {
                        return ("FileNotFound <:b>".to_string(), false);
                    }
                } else {
                    return ("NoFileNameProvided <:b>".to_string(), false);
                }
            }

            // Very problematic, and a bery bad idea at that.
            // "bd" | "BD" | "Bd" | "bD" => {
            //     if let Some(param) = parameter {
            //         let r = efs.baptize_dir(param);

            //         if !r {
            //             return ("DirectoryNotFound <:bd>".to_string(), false);
            //         }
            //     }
            // }

            "f" | "F" => {
                if let Some(param) = parameter {                
                    let r = find_word_in_text(param, &text, cursor);
                    
                    if !r {
                        return ("IdentifierNotFound <:f>".to_string(), false);
                    }
                    
                } else {
                    return ("NoIdentifierProvided <:f>".to_string(), false);
                }
            }

            "r" | "R" => {
                if let Some(param) = parameter {
                    let r = efs.delete_file(param);

                    if !r {
                        return ("FileNotFound <:r>".to_string(), false);
                    }
                } else {
                    return ("NoFileNameProvided <:r>".to_string(), false);
                }
            }

            "md" | "Md" | "mD" | "MD" => {
               if let Some(param) = parameter {
                  if !efs.create_dir(param) {
                     return ("DirectoryNameUsed <:md>".to_string(), false);
                  }
               } else {
                  return ("NoDirectoryProvided <:md>".to_string(), false);
               }
            }

            "rd" | "Rd" | "RD" | "rD" => {
               if let Some(param) = parameter {
                  if !efs.delete_dir(param) {
                     return ("DirectoryNotFound <:rd>".to_string(), false);
                  }
               } else {
                  return ("NoDirectoryProvided <:rd>".to_string(), false);
               }
            }
            
            "c" | "C" => {
                if let Some(param) = parameter {
                    let r = efs.create_file(param);

                    if !r {
                        return ("FileNameUsed <:c>".to_string(), false);
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
                            return ("FileNotFound <:cd>".to_string(), false);
                        }
                    }
                } else {
                    return ("NoDirectoryNameProvided <:cd>".to_string(), false);
                }
            }

            "l" | "L" => {
               if let Some(param) = parameter {
                  if param.parse::<u64>().is_ok() == false {
                    return ("InvalidLineArgument <:l>".to_string(), false);
                  }

                  let new_line_num: u32 = FromStr::from_str(param).unwrap();            
                  let new_line: usize = new_line_num as usize;

                  cursor.xy.1 = new_line;
                  cursor.xy.0 = 0;
               } else {
                  return ("NoLineNumProvided <:l>".to_string(), false);
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
            "ectrl"         => return (console_manual(5), true),
            "ever"          => return (VERSION.to_string(), false),

            "egam" | "rand" | "roll" => {
                if let Some(param) = parameter {
                    if param.parse::<u32>().is_ok() == false {
                        return ("InvalidGambleArgument <:egam>".to_string(), false);
                    }

                    let max_num: u32 = FromStr::from_str(param).unwrap();
                    let rand_num = rand::rand() as u32 % (max_num + 1);

                    return (format!("Gamble result: {}", rand_num), false);
                } else {
                    return ("NoMaxNumProvided <:egam>".to_string(), false);
                }
            }

            // Options
            "eau" => {
                ops.toggle_audio();
            }

            "esm" => {
                ops.toggle_smart();                
            }
            
            "efl" => {
                ops.toggle_fullscreen();    
            }
            
            "ehi" => {
                ops.toggle_highlight();    
            }

            _ => return ("UnknownDirective".to_string(), false),
        }
    } else {
        // File switch
        if efs.change_current_file(directive.to_string()) {
            text.clear();
            *text = efs.load_current_file().unwrap_or_default();

            let fname = path_buffer_file_to_string(&efs.current_file);

            let ext = Path::new(&fname)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            // Load the new language support
            *elk = load_keywords_for_extension(ext);
        } else {
            text.clear();
            efs.current_file = None;
            return ("FileNotFound".to_string(), false);
        }
    }

    *directive = String::new();

    return ("".to_string(), false);
}
