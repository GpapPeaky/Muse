use rfd::*;

use std::path::{Path, PathBuf};

pub struct EditorFileSystem {
    pub current_dir: Option<PathBuf>,
    pub current_file: Option<PathBuf>
}

impl EditorFileSystem {
    pub fn new() -> Self {
        EditorFileSystem {
            current_dir: None,
            current_file: None
        }
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
            // TODO: throw console message here
        }
    }

    /// Change to another cwd, cd use ,
    /// returns true if the change was valid, else false
    pub fn change_current_directory(&mut self, p: impl AsRef<Path>) -> bool {
        let base = self.current_dir.clone().unwrap_or_else(|| std::env::current_dir().unwrap());
        let new_path = base.join(p.as_ref());
    
        match std::fs::canonicalize(&new_path) {
            Ok(valid_path) if valid_path.is_dir() => {
                println!("Changed to: {}", valid_path.display());
                std::env::set_current_dir(&valid_path).ok();
                self.current_dir = Some(valid_path);
                true
            }
            Ok(valid_path) => {
                eprintln!("Not a dir: {}", valid_path.display());
                false
            }
            Err(e) => {
                eprintln!("Invalid path {:?}: {}", new_path, e);
                false
            }
        }
    }

    /// Change to another file inside the current directory
    /// by typing its name in the console
    /// returns true if the change was valid, else false
    pub fn change_current_file(&mut self, f: String) -> bool {
        if let Some(dir) = &self.current_dir {
            let file_path = dir.join(&f);
            if file_path.is_file() {
                self.current_file = Some(file_path);
                return true;
            }
        }

        false
    }
}

/// Get a path buffer as a string
pub fn path_buffer_to_string(p: &Option<std::path::PathBuf>) -> String {
    match p {
        Some(path) => path.display().to_string(),
        None => "</>".to_string(),
    }
}

/// Get the path to the file, then trim it to only get the file text
pub fn path_buffer_file_to_string(pb: &Option<PathBuf>) -> String {
    if let Some(path) = pb {
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    }
}
