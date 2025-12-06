use macroquad::prelude::*;
use rfd::*;

use std::{fs, io, path::{Path, PathBuf}};
use std::io::Write;

use crate::console::editor_console::*;
use crate::options::editor_pallete::*;

pub struct EditorFileSystem {
    pub current_dir: Option<PathBuf>,
    pub current_file: Option<PathBuf>,
    pub unsaved_changes: bool
}

impl EditorFileSystem {
    pub fn new() -> Self {
        EditorFileSystem {
            current_dir: None,
            current_file: None,
            unsaved_changes: false
        }
    }

    /// Load the contents of the currently open file
    pub fn load_current_file(&self) -> io::Result<Vec<String>> {
        if let Some(ref file) = self.current_file {
            let path = self.current_dir.as_ref().unwrap_or(&std::env::current_dir().unwrap()).join(file);
            let content = fs::read_to_string(path)?;
            
            Ok(content.lines().map(|s| s.to_string()).collect())
        } else {
            Ok(vec![])  // no file selected
        }
    }

    /// Write a Vec<String> back to the current file
    pub fn write_current_file(&mut self, text: &[String]) -> io::Result<()> {
        if let Some(ref file) = self.current_file {
            let path = self.current_dir.as_ref().unwrap_or(&std::env::current_dir().unwrap()).join(file);
            let mut f = fs::File::create(path)?;

            for line in text {
                writeln!(f, "{}", line)?;
            }

            self.unsaved_changes = false;
        }

        Ok(())
    }

    /// Open native file explorer, via the Rust File Dialog
    /// crate.
    pub fn open_file_explorer(&mut self) {
        let mut dialog = FileDialog::new();

        // If we already have a current directory, set it as the starting directory
        if let Some(dir) = &self.current_dir {
            dialog = dialog.set_directory(dir);
        }

        // Open a folder picker dialog
        if let Some(folder) = dialog.pick_folder() {
            self.current_dir = Some(folder);
        } else {
            // User cancelled the dialog
            // Do nothing
        }
    }

    /// Change to another cwd, cd use ,
    /// returns true if the change was valid, else false
    pub fn change_current_directory(
        &mut self,
        p: impl AsRef<Path>
    ) -> bool {
        let base = self.current_dir.clone().unwrap_or_else(|| std::env::current_dir().unwrap());
        let new_path = base.join(p.as_ref());
    
        match std::fs::canonicalize(&new_path) {
            Ok(valid_path) if valid_path.is_dir() => {
                // println!("Changed to: {}", valid_path.display());
                std::env::set_current_dir(&valid_path).ok();
                self.current_dir = Some(valid_path);
                true
            }

            #[allow(unused_variables)]
            Ok(valid_path) => {
                // eprintln!("Not a dir: {}", valid_path.display());
                false
            }

            #[allow(unused_variables)]
            Err(e) => {
                // eprintln!("Invalid path {:?}: {}", new_path, e);
                false
            }
        }
    }

    /// Change to another file inside the current directory
    /// by typing its name in the console
    /// returns true if the change was valid, else false
    pub fn change_current_file(
        &mut self,
        f: String
    ) -> bool {
        let Some(dir) = &self.current_dir else {
            return false;
        };
    
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return false,
        };
    
        for entry in entries.flatten() {
            let path = entry.path();
    
            // Only match if it's a file and the name matches EXACTLY (case-sensitive)
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name == f {
                        self.current_file = Some(path);
                        return true;
                    }
                }
            }
        }
    
        false
    }

    /// Create a file of name <fname>
    /// returns true if it was successful
    /// false if not, or if the file with that name already exists
    pub fn create_file(
        &mut self,
        fname: &str
    ) -> bool {
        let Some(dir) = &self.current_dir else {
            return false;
        };

        let mut newfile = dir.clone();
        newfile.push(fname);

        match fs::OpenOptions::new()
            .write(true)
            .create_new(true) // fails if the file already exists
            .open(&newfile)
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Delete a file of name <fname>
    /// returns true if it was successful
    pub fn delete_file(
        &mut self,
        fname: &str
    ) -> bool {
        let Some(dir) = &self.current_dir else {
            return false;
        };

        let mut targetfile = dir.clone();
        targetfile.push(fname);

        match fs::remove_file(&targetfile) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

   /// Create a new directory.
   /// Returns true if the name is valid, false if not
    pub fn create_dir(
        &mut self,
        dname: &str
    ) -> bool {
      let base = match &self.current_dir {
         Some(p) => p.clone(),
         
         None => match std::env::current_dir() {
            Ok(p) => p,
            Err(_) => return false
         },
      };

      let folder_path = base.join(dname);

      fs::create_dir_all(folder_path).is_ok()
   }
   

   /// Delete a directory
   /// Returns true if possible, false if not
    pub fn delete_dir(
        &mut self,
        dname: &str
    ) -> bool {
      let base = match &self.current_dir {
         Some(p) => p.clone(),
         
         None => match std::env::current_dir() {
            Ok(p) => p,
            Err(_) => return false
         },
      };

      let folder_name = base.join(dname);

      if !folder_name.exists() {
         return false;
      }

      fs::remove_dir_all(folder_name).is_ok()
    }

    /// Rename the current open file to fname
    /// return true if complete, false if not
    pub fn baptize_file(
        &mut self,
        fname: &str
    ) -> bool {
        let old_path = match &self.current_file {
            Some(p) => p.clone(),
            None => return false,
        };

        let mut new_path = old_path.clone();
        new_path.set_file_name(fname);

        if std::fs::rename(&old_path, &new_path).is_err() {
            return false;
        }

        self.current_file = Some(new_path);

        true
    }

    // /// Rename the current open file to folder
    // /// return true if complete, false if not
    // pub fn baptize_dir(&mut self, dname: &str) -> bool {
    //     let old_path = match &self.current_dir {
    //         Some(p) => p.clone(),
    //         None => return false, // no current directory
    //     };

    //     if old_path.file_name()
    //         .map(|n| n == dname)
    //         .unwrap_or(false)
    //     {
    //         return true;
    //     }

    //     let mut new_path = old_path.clone();
    //     new_path.set_file_name(dname);

    //     if new_path.exists() {
    //         return false; // cannot rename, target already exists
    //     }

    //     // Attempt rename
    //     if std::fs::rename(&old_path, &new_path).is_err() {
    //         return false; 
    //     }

    //     // Update internal reference
    //     self.current_dir = Some(new_path);

    //     true
    // }
}

/// Get a path buffer as a string
pub fn path_buffer_to_string(
    p: &Option<std::path::PathBuf>
) -> String {
    match p {
        Some(path) => path.display().to_string(),
        None => "</>".to_string(),
    }
}

/// Get the path to the file, then trim it to only get the file text
pub fn path_buffer_file_to_string(
    pb: &Option<PathBuf>
) -> String {
    if let Some(path) = pb {
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    }
}

/// Display files and folders in the current working directory.
/// Highlights the currently open file.
/// When typing in the console, only the ones matching the text input will be shown.
/// Returns the closet matching filename/directory for autocompletion when TAB is pressed.
pub fn draw_dir_contents(
    current_file: &Option<PathBuf>,
    current_dir: &Option<PathBuf>,
    directive: &str,
    console: &EditorConsole,
    is_cd: bool,
) -> String {
    let Some(dir) = current_dir else {
        return "".to_string();
    };

    let entries = match fs::read_dir(dir) {
        Ok(v) => v,
        Err(_) => return "".to_string(),
    };

    // Make to lowercase
    let directive_lc = directive.to_lowercase();
    
    // Extract query text
    let query = if directive.is_empty() {
        "".to_string()
    } else if directive_lc.starts_with(":cd ") {
        directive[4..].to_lowercase()
    } else if directive.starts_with(':') {
        directive.split_whitespace().last().unwrap_or("").to_lowercase()
    } else {
        directive.to_lowercase()
    };
    
    // Mode flags
    let show_all = directive.is_empty();
    let show_dirs_only = directive_lc.starts_with(":cd ");
    let show_files_only = !show_all && !show_dirs_only;

    let mut best_match = String::new();
    let mut y = 50.0 + CONSOLE_MARGINS;
    let x = screen_width() - console.width + CONSOLE_MARGINS;

    for entry in entries.flatten() {
        let path = entry.path();
        let name_os = entry.file_name();
        let mut name = name_os.to_string_lossy().to_string();
        let name_lc = name.to_lowercase();
        let is_dir = path.is_dir();

        // Filtering logic
        if !show_all {
            if is_cd && !is_dir {
                continue;
            }
            if !is_cd && is_dir {
                continue;
            }
            
            // Matching names
            if !name_lc.contains(&query) {
                continue;
            }
        }

        // Autocomplete match
        if best_match.is_empty() && !show_all {
            if show_dirs_only && is_dir {
                best_match = name.clone();
            } else if show_files_only && !is_dir {
                best_match = name.clone();
            }
        }

        // Highlight and formatting
        let color = if Some(&path) == current_file.as_ref() {
            SELECTED_FILE_COLOR
        } else if is_dir {
            FOLDER_COLOR
        } else {
            FILE_COLOR
        };

        if is_dir {
            name.push('/');
        }

        draw_text(&name, x, y, 24.0, color);
        y += 20.0;
    }

    if is_key_pressed(KeyCode::Tab) {
        return best_match;
    }

    "".to_string()
}

