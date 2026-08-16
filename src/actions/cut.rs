use std::io::Read;
use std::path::Path;

use super::copy::fs_err_canonicalize;
use crate::config::Config;
use crate::gui::GuiClipboard;
use crate::storage::ClipboardStorage;
use crate::utils;

pub fn execute(
    items: &[String],
    config: &Config,
    storage: &ClipboardStorage,
    gui: &GuiClipboard,
) -> Result<(), String> {
    storage.ensure_dirs().map_err(|e| e.to_string())?;

    if !items.is_empty() {
        let all_exist = items.iter().all(|item| Path::new(item).exists());
        if all_exist {
            // Cut files/directories
            storage.clear_data_dir().map_err(|e| e.to_string())?;
            let mut abs_paths = Vec::new();

            for item in items {
                let src = Path::new(item);
                let abs = fs_err_canonicalize(src);
                abs_paths.push(abs);

                let file_name = src
                    .file_name()
                    .ok_or_else(|| format!("Invalid file path: {}", item))?;
                let dst = storage.data_dir.join(file_name);

                utils::copy_recursive(src, &dst, config.fast_copy)
                    .map_err(|e| format!("Failed to copy {}: {}", item, e))?;
            }

            storage
                .record_originals(&abs_paths)
                .map_err(|e| e.to_string())?;
            storage
                .record_last_copied(&abs_paths)
                .map_err(|e| e.to_string())?;
            gui.set_files(&abs_paths);

            if !config.silent && !config.no_confirmation {
                println!("Cut {} item(s)", items.len());
            }
            return Ok(());
        } else {
            // Cut text
            let text = items.join(" ");
            storage.set_raw_text(&text).map_err(|e| e.to_string())?;
            storage
                .record_originals(std::slice::from_ref(&storage.raw_data_file))
                .map_err(|e| e.to_string())?;
            gui.set_text(&text);

            if !config.silent && !config.no_confirmation {
                let preview = if text.len() > 60 {
                    format!("{}...", &text[..60])
                } else {
                    text.clone()
                };
                println!("Cut text \"{}\"", preview);
            }
            return Ok(());
        }
    }

    // Cut from stdin
    if !utils::is_stdin_terminal() {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| format!("Failed to read stdin: {}", e))?;

        storage.set_raw_text(&buffer).map_err(|e| e.to_string())?;
        storage
            .record_originals(std::slice::from_ref(&storage.raw_data_file))
            .map_err(|e| e.to_string())?;
        gui.set_text(&buffer);

        if !config.silent && !config.no_confirmation {
            println!("Cut piped input ({} bytes)", buffer.len());
        }
        Ok(())
    } else {
        Err("No items or text provided to cut.".to_string())
    }
}
