use std::fs;
use std::io::Write;
use std::process::Command;

use crate::config::Config;
use crate::gui::GuiClipboard;
use crate::storage::ClipboardStorage;

pub fn execute(
    config: &Config,
    storage: &ClipboardStorage,
    gui: &GuiClipboard,
) -> Result<(), String> {
    storage.ensure_dirs().map_err(|e| e.to_string())?;

    // Determine initial text
    let initial_text = if storage.holds_raw_data() {
        storage.get_raw_text().unwrap_or_default()
    } else if storage.holds_files() {
        let files = storage.get_files().map_err(|e| e.to_string())?;
        if files.len() == 1 && files[0].is_file() {
            match fs::read_to_string(&files[0]) {
                Ok(content) => content,
                Err(_) => {
                    return Err(
                        "You can only edit text content. Try copying some text instead."
                            .to_string(),
                    );
                }
            }
        } else {
            return Err(
                "You can only edit text content. Try copying some text instead.".to_string(),
            );
        }
    } else {
        // Sync from GUI if available
        gui.get_text().unwrap_or_default()
    };

    let editor_cmd = find_editor(config)?;

    // Create a temporary file with the current text
    let mut temp_file = tempfile::Builder::new()
        .prefix("clb_edit_")
        .suffix(".txt")
        .tempfile()
        .map_err(|e| format!("Failed to create temporary file: {}", e))?;

    temp_file
        .write_all(initial_text.as_bytes())
        .map_err(|e| format!("Failed to write to temporary file: {}", e))?;
    temp_file
        .flush()
        .map_err(|e| format!("Failed to flush temporary file: {}", e))?;

    let temp_path = temp_file.path().to_path_buf();

    // Parse editor command and arguments
    let parts: Vec<&str> = editor_cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Invalid editor configuration.".to_string());
    }

    let program = parts[0];
    let args = &parts[1..];

    let mut cmd = Command::new(program);
    cmd.args(args);
    cmd.arg(&temp_path);

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to run editor '{}': {}", editor_cmd, e))?;

    if !status.success() {
        // Canceled edit (e.g. :cq in vim), leave clipboard alone
        if !config.silent {
            println!("Edit canceled; clipboard was not modified.");
        }
        return Ok(());
    }

    // Read back edited contents
    let edited_text = fs::read_to_string(&temp_path)
        .map_err(|e| format!("Failed to read edited temporary file: {}", e))?;

    if edited_text != initial_text {
        storage
            .set_raw_text(&edited_text)
            .map_err(|e| e.to_string())?;
        gui.set_text(&edited_text);

        if !config.silent && !config.no_confirmation {
            println!("Edited clipboard text");
        }
    }

    Ok(())
}

fn find_editor(config: &Config) -> Result<String, String> {
    if let Some(ref editor) = config.editor {
        let trimmed = editor.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    let fallbacks = ["nano", "vim", "vi", "notepad", "code", "subl", "emacs"];
    for editor in fallbacks {
        if which::which(editor).is_ok() {
            return Ok(editor.to_string());
        }
    }

    Err(
        "No suitable editor found. Set the CLIPBOARD_EDITOR or EDITOR environment variable."
            .to_string(),
    )
}
