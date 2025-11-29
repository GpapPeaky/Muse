// Editor's 'virtual' terminal

use std::process::{Command, Stdio};
use std::path::PathBuf;

/// EXPERIMENTAL: Execute terminal commands, with the <:t> $ <command> directive
pub fn execute_terminal_command(directive_command: &str, current_dir: &Option<PathBuf>) -> (String, bool) {
    // Extract command after $
    let t_command = directive_command
        .split('$')
        .nth(1)
        .map(|s| s.trim());

    match t_command {
        Some(cmd) if !cmd.is_empty() => {
            if let Some(dir) = current_dir {
                // Split command into program + args manually
                let mut parts = cmd.split_whitespace();
                if let Some(program) = parts.next() {
                    let args: Vec<&str> = parts.collect();

                    let output = Command::new(program)
                        .args(&args)
                        .current_dir(&dir)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output();

                    match output {
                        Ok(output) => {
                            let mut result = String::new();
                            if !output.stdout.is_empty() {
                                result.push_str(&String::from_utf8_lossy(&output.stdout));
                            }
                            if !output.stderr.is_empty() {
                                if !result.is_empty() { result.push('\n'); }
                                result.push_str(&String::from_utf8_lossy(&output.stderr));
                            }
                            (result.to_string(), true)
                        }
                        Err(e) => (format!("Failed to execute command: {}", e), false),
                    }
                } else {
                    ("NoCommandGiven <:t>".to_string(), false)
                }
            } else {
                ("InvalidPathForCommand <:t>".to_string(), false)
            }
        }
        _ => ("NoCommandGiven <:t>".to_string(), false),
    }
}