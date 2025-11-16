# Muse Code Text Editor (v1.4.0)

A lightweight console-based editor that allows users to manage files, directories, and editor configurations directly from a command-line interface. The editor supports switching between console mode and insert mode seamlessly, along with a wide range of file, directory, and editor management directives.

<p align="center">
  <img src = "assets/icon/mdmuse.png"/>
</p>


## Table of Contents

- [Modes](#modes)
- [Text Navigation](#text-navigation)
- [Console Directives](#console-directives)
  - [File Directives](#file-directives)
  - [Directory Directives](#directory-directives)
  - [Configuration Directives](#configuration-directives)
  - [Other Directives](#other-directives)
- [Usage](#usage)
- [Autocomplete](#autocomplete)
- [License](#license)

---

## Modes

The editor has two main modes:

- **Console Mode** – Execute directives, switch files/directories, manage configurations.
- **Insert Mode** – Edit file content.

Switch between modes by pressing: *LCtrl + `*

---

## Text Navigation

We can navigate the current file's text in these ways:

| Keys | Description |
|-----------|-------------|
| `ArrowKeys` | Move the cursor index in the file 1 at a time. |
| `LCtrl + ArrowKeys` | Move the cursor index in the file 5 steps at a time. |
| `LCtrl + LShift+ ArrowKeys` | Move the cursor index continously up and down, 1 index increment/decrement at a time. |
| `:l <N>` | Go to line `N` in the current file. Throws an error if invalid. |

---

## Console Directives

All console commands are prefixed with a `:`. Commands without `:` are treated as **switch-to-file or directory commands**.

---

### File Directives

| Directive | Description |
|-----------|-------------|
| `:l <N>` | Go to line `N` in the current file. Throws an error if invalid. |
| `:w` | Write/save the current file. |
| `:i` | Display information about the current file. |
| `:r <f>` | Remove the file named `<f>`. |
| `:b <f>` | Rename the current file to `<f>`. |
| `:f <f>` | Jump to the line where the first occurrence of text `<f>` exists. |
| `:c <f>` | Create a new file named `<f>`. |

---

### Directory Directives

| Directive | Description |
|-----------|-------------|
| `:cd` | Change the current directory. |
| `:od/:o` | Open the directory in the native file explorer. |
| `:md <f>` | Create a new directory named `<f>`. |
| `:rd <f>` | Remove a directory named `<f>` and all its contents. |

---

### Configuration Directives

All configuration changes are saved in `cal.conf`.

| Directive | Description |
|-----------|-------------|
| `:epa <p>` | Change the editor palette to `<p>`. |
| `:efn <p>` | Change the editor font to `<p>`. |
| `:eau` | Toggle editor audio on/off. |
| `:eav <N>` | Set editor audio volume to `N`. |

---

### Other Directives

| Directive | Description |
|-----------|-------------|
| `:e/q` | Exit/close the editor. |
| `:egman` | Display the general editor manual. |
| `:efman` | Display file directive manual. |
| `:edman` | Display directory directive manual. |
| `:ecman` | Display editor configuration manual. |
| `:eoman` | Display other editor directives manual. |
| `:ectrl` | Display the editor infile controls manual. |
| `:ever` | Display editor version. |
| `:egam/:rand/:roll <N>` | Display a random number between 0 and `N`. |

---

## Usage

1. Start the editor and open the console.
2. Use directives to navigate files, directories, or configure the editor.
3. Switch to insert mode for editing text using: *LCtrl + `*
4. Save and manage files using the `:w`, `:c`, `:r`, or `:b` directives.

---

## Autocomplete

- While typing a file or directory name, the console will display similar existing names.
- Press `TAB` to autocomplete the first match.

---

## License

Non-Commercial Free Software License (NC-FSL) v1.0, see LICENCE.md for more info.

---
