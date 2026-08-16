use std::fs;
use std::io::Write;
use std::path::PathBuf;

use regex::Regex;

use crate::config::Config;
use crate::gui::GuiClipboard;
use crate::storage::ClipboardStorage;
use crate::utils;

pub fn execute(
    patterns: &[String],
    config: &Config,
    storage: &ClipboardStorage,
    gui: &GuiClipboard,
) -> Result<(), String> {
    storage.ensure_dirs().map_err(|e| e.to_string())?;

    // Build regex filters if provided
    let regexes: Result<Vec<Regex>, regex::Error> =
        patterns.iter().map(|p| Regex::new(p)).collect();
    let regexes = regexes.map_err(|e| format!("Invalid regex pattern: {}", e))?;

    // Sync from GUI if storage has no data or GUI text is present
    if !storage.holds_files()
        && let Some(gui_text) = gui.get_text()
    {
        let stored = storage.get_raw_text();
        match stored {
            Some(stored_text) if gui_text != stored_text => {
                let _ = storage.set_raw_text(&gui_text);
            }
            None if !gui_text.is_empty() => {
                let _ = storage.set_raw_text(&gui_text);
            }
            _ => {}
        }
    }

    if !storage.holds_data() {
        return Err("Clipboard is empty.".to_string());
    }

    let originals = storage.get_originals();

    // If storage holds raw text: print text to stdout
    if storage.holds_raw_data() {
        if let Some(text) = storage.get_raw_text() {
            if !utils::is_stdout_terminal() {
                print!("{}", text);
                let _ = std::io::stdout().flush();
            } else {
                print!("{}", text);
                if !text.ends_with('\n') {
                    println!();
                }
                let _ = std::io::stdout().flush();
            }
        }
        if let Some(orig_paths) = originals {
            cleanup_originals(&orig_paths, storage);
        }
        return Ok(());
    }

    // Storage holds files: paste into current working directory
    let files = storage.get_files().map_err(|e| e.to_string())?;
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let mut pasted_count = 0;

    for src in &files {
        let file_name = src
            .file_name()
            .ok_or_else(|| format!("Invalid source file: {}", src.display()))?;
        let name_str = file_name.to_string_lossy().to_string();

        if !matches_regexes(&name_str, &regexes) {
            continue;
        }

        let dst = cwd.join(file_name);
        utils::copy_recursive(src, &dst, config.fast_copy)
            .map_err(|e| format!("Failed to paste {}: {}", src.display(), e))?;
        pasted_count += 1;
    }

    if let Some(orig_paths) = originals {
        cleanup_originals(&orig_paths, storage);
    }

    if !config.silent && !config.no_confirmation {
        println!("Pasted {} item(s)", pasted_count);
    }

    Ok(())
}

fn matches_regexes(name: &str, regexes: &[Regex]) -> bool {
    if regexes.is_empty() {
        return true;
    }
    regexes.iter().any(|r| r.is_match(name))
}

pub fn cleanup_originals(paths: &[PathBuf], storage: &ClipboardStorage) {
    for path in paths {
        if path == &storage.raw_data_file {
            let _ = storage.clear_data_dir();
        } else if path.exists() {
            if path.is_dir() {
                let _ = fs::remove_dir_all(path);
            } else {
                let _ = fs::remove_file(path);
            }
        }
    }
    let _ = storage.clear_originals();
}
