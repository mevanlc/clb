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

        #[cfg(target_os = "macos")]
        {
            if macos::set_files(paths) {
                return true;
            }
        }

        // Write file paths as text fallback
        let text = paths
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        self.set_text(&text)
    }

    pub fn get_files(&self) -> Option<Vec<PathBuf>> {
        if !self.enabled {
            return None;
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(files) = macos::get_files()
                && !files.is_empty()
            {
                return Some(files);
            }
        }

        None
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

#[cfg(target_os = "macos")]
mod macos {
    use objc2::ClassType;
    use objc2::runtime::ProtocolObject;
    use objc2_app_kit::{NSPasteboard, NSPasteboardWriting};
    use objc2_foundation::{NSArray, NSDictionary, NSString, NSURL};
    use std::path::PathBuf;

    pub fn set_files(paths: &[PathBuf]) -> bool {
        let pboard = NSPasteboard::generalPasteboard();
        pboard.clearContents();

        let mut writing_objects = Vec::new();
        for path in paths {
            let path_str = path.to_string_lossy();
            let ns_path = NSString::from_str(&path_str);
            let url = NSURL::fileURLWithPath(&ns_path);
            let proto: objc2::rc::Retained<ProtocolObject<dyn NSPasteboardWriting>> =
                ProtocolObject::from_retained(url);
            writing_objects.push(proto);
        }

        let ns_array = NSArray::from_retained_slice(&writing_objects);
        pboard.writeObjects(&ns_array)
    }

    pub fn get_files() -> Option<Vec<PathBuf>> {
        let pboard = NSPasteboard::generalPasteboard();
        let classes = NSArray::from_slice(&[NSURL::class()]);
        let options = NSDictionary::new();
        let objects = unsafe { pboard.readObjectsForClasses_options(&classes, Some(&options)) }?;

        let mut paths = Vec::new();
        for obj in objects.iter() {
            if let Some(url) = obj.downcast_ref::<NSURL>()
                && let Some(path_ns) = url.path()
            {
                let path_str = path_ns.to_string();
                if !path_str.is_empty() {
                    paths.push(PathBuf::from(path_str));
                }
            }
        }

        if paths.is_empty() { None } else { Some(paths) }
    }
}
