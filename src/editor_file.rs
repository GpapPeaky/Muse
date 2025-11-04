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
    pub fn change_current_directory(&mut self, p: String) -> bool {
        let path = Path::new(&p);

        if path.is_dir() {
            self.current_dir = Some(path.to_path_buf());

            true
        } else {
            false
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
