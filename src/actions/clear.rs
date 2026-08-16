use std::fs;

use crate::config::Config;
use crate::gui::GuiClipboard;
use crate::storage::ClipboardStorage;

pub fn execute(
    entries: &[String],
    all: bool,
    config: &Config,
    storage: &ClipboardStorage,
    gui: &GuiClipboard,
) -> Result<(), String> {
    if all {
        let mut count = 0;
        if let Ok(dir_entries) = fs::read_dir(&config.temp_dir) {
            for entry in dir_entries.flatten() {
                let _ = fs::remove_dir_all(entry.path());
                count += 1;
            }
        }
        if let Ok(dir_entries) = fs::read_dir(&config.persist_dir) {
            for entry in dir_entries.flatten() {
                let _ = fs::remove_dir_all(entry.path());
                count += 1;
            }
        }
        gui.clear();

        if !config.silent && !config.no_confirmation {
            println!("Cleared {} clipboard(s)", count);
        }
        return Ok(());
    }

    if !entries.is_empty() {
        for entry_str in entries {
            let entry_dir = storage.root.join("data").join(entry_str);
            if entry_dir.exists() {
                let _ = fs::remove_dir_all(entry_dir);
            }
        }
    } else {
        storage.clear().map_err(|e| e.to_string())?;
        gui.clear();
    }

    if !config.silent && !config.no_confirmation {
        println!("Cleared clipboard");
    }

    Ok(())
}
