use crate::config::Config;
use crate::gui::GuiClipboard;
use crate::storage::ClipboardStorage;

use super::paste;

pub fn execute(
    patterns: &[String],
    config: &Config,
    storage: &ClipboardStorage,
    gui: &GuiClipboard,
) -> Result<(), String> {
    storage.ensure_dirs().map_err(|e| e.to_string())?;

    // Treat last copy as cut:
    // If we have recorded last copied paths, set them as originals
    if let Some(last_copied) = storage.get_last_copied() {
        storage
            .record_originals(&last_copied)
            .map_err(|e| e.to_string())?;
    } else if storage.holds_raw_data() {
        storage
            .record_originals(std::slice::from_ref(&storage.raw_data_file))
            .map_err(|e| e.to_string())?;
    }

    // Now execute paste, which will paste and remove the originals
    paste::execute(patterns, config, storage, gui)
}
