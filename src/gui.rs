use crate::config::Config;
use std::path::PathBuf;

pub struct GuiClipboard {
    enabled: bool,
}

impl GuiClipboard {
    pub fn new(config: &Config) -> Self {
        Self {
            enabled: config.gui_enabled,
        }
    }

    pub fn get_text(&self) -> Option<String> {
        if !self.enabled {
            return None;
        }
        let mut cb = arboard::Clipboard::new().ok()?;
        cb.get_text().ok()
    }

    pub fn set_text(&self, text: &str) -> bool {
        if !self.enabled {
            return false;
        }
        if let Ok(mut cb) = arboard::Clipboard::new() {
            cb.set_text(text).is_ok()
        } else {
            false
        }
    }

    pub fn set_files(&self, paths: &[PathBuf]) -> bool {
        if !self.enabled {
            return false;
        }
        // Write file paths as text fallback for arboard
        let text = paths
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        self.set_text(&text)
    }

    pub fn clear(&self) -> bool {
        if !self.enabled {
            return false;
        }
        if let Ok(mut cb) = arboard::Clipboard::new() {
            cb.clear().is_ok()
        } else {
            false
        }
    }
}
