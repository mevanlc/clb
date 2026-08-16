use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::config::Config;

const DATA_DIR_NAME: &str = "data";
const METADATA_DIR_NAME: &str = "metadata";
const RAW_DATA_FILE_NAME: &str = "rawdata.clipboard";
const ORIGINALS_FILE_NAME: &str = "originals";
const LAST_COPIED_FILE_NAME: &str = "last_copied";
const NOTES_FILE_NAME: &str = "notes";
const VERSION_FILE_NAME: &str = "version";
const LOCK_FILE_NAME: &str = "lock";

#[derive(Debug, Clone)]
pub struct ClipboardStorage {
    pub name: String,
    pub entry: usize,
    pub root: PathBuf,
    pub data_dir: PathBuf,
    pub raw_data_file: PathBuf,
    pub metadata_dir: PathBuf,
    pub is_persistent: bool,
}

impl ClipboardStorage {
    pub fn new(config: &Config) -> Self {
        let is_persistent = config.clipboard_name.contains('_');
        let base_dir = if is_persistent {
            &config.persist_dir
        } else {
            &config.temp_dir
        };

        let root = base_dir.join(&config.clipboard_name);
        let entry_str = config.clipboard_entry.to_string();
        let data_dir = root.join(DATA_DIR_NAME).join(&entry_str);
        let raw_data_file = data_dir.join(RAW_DATA_FILE_NAME);
        let metadata_dir = root.join(METADATA_DIR_NAME);

        Self {
            name: config.clipboard_name.clone(),
            entry: config.clipboard_entry,
            root,
            data_dir,
            raw_data_file,
            metadata_dir,
            is_persistent,
        }
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.data_dir)?;
        fs::create_dir_all(&self.metadata_dir)?;
        let version_path = self.metadata_dir.join(VERSION_FILE_NAME);
        if !version_path.exists() {
            fs::write(version_path, "1\n")?;
        }
        Ok(())
    }

    pub fn holds_raw_data(&self) -> bool {
        if let Ok(meta) = fs::metadata(&self.raw_data_file) {
            meta.is_file() && meta.len() > 0
        } else {
            false
        }
    }

    pub fn holds_files(&self) -> bool {
        if let Ok(entries) = fs::read_dir(&self.data_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                if name != RAW_DATA_FILE_NAME {
                    return true;
                }
            }
        }
        false
    }

    pub fn holds_data(&self) -> bool {
        self.holds_raw_data() || self.holds_files()
    }

    pub fn get_raw_text(&self) -> Option<String> {
        if self.holds_raw_data() {
            fs::read_to_string(&self.raw_data_file).ok()
        } else {
            None
        }
    }

    pub fn set_raw_text(&self, text: &str) -> std::io::Result<()> {
        self.ensure_dirs()?;
        // Clear existing files in data_dir
        self.clear_data_dir()?;
        fs::write(&self.raw_data_file, text)
    }

    pub fn clear_data_dir(&self) -> std::io::Result<()> {
        if self.data_dir.exists() {
            for entry in fs::read_dir(&self.data_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(path)?;
                } else {
                    fs::remove_file(path)?;
                }
            }
        }
        Ok(())
    }

    pub fn get_files(&self) -> std::io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        if self.data_dir.exists() {
            for entry in fs::read_dir(&self.data_dir)? {
                let entry = entry?;
                let name = entry.file_name();
                if name != RAW_DATA_FILE_NAME {
                    files.push(entry.path());
                }
            }
        }
        files.sort();
        Ok(files)
    }

    pub fn record_originals(&self, paths: &[PathBuf]) -> std::io::Result<()> {
        self.ensure_dirs()?;
        let orig_file = self.metadata_dir.join(ORIGINALS_FILE_NAME);
        let mut file = fs::File::create(orig_file)?;
        for path in paths {
            writeln!(file, "{}", path.display())?;
        }
        Ok(())
    }

    pub fn get_originals(&self) -> Option<Vec<PathBuf>> {
        let orig_file = self.metadata_dir.join(ORIGINALS_FILE_NAME);
        if orig_file.is_file() {
            let content = fs::read_to_string(orig_file).ok()?;
            let paths: Vec<PathBuf> = content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(PathBuf::from)
                .collect();
            if paths.is_empty() { None } else { Some(paths) }
        } else {
            None
        }
    }

    pub fn clear_originals(&self) -> std::io::Result<()> {
        let orig_file = self.metadata_dir.join(ORIGINALS_FILE_NAME);
        if orig_file.exists() {
            fs::remove_file(orig_file)?;
        }
        Ok(())
    }

    pub fn record_last_copied(&self, paths: &[PathBuf]) -> std::io::Result<()> {
        self.ensure_dirs()?;
        let last_file = self.metadata_dir.join(LAST_COPIED_FILE_NAME);
        let mut file = fs::File::create(last_file)?;
        for path in paths {
            writeln!(file, "{}", path.display())?;
        }
        Ok(())
    }

    pub fn get_last_copied(&self) -> Option<Vec<PathBuf>> {
        let last_file = self.metadata_dir.join(LAST_COPIED_FILE_NAME);
        if last_file.is_file() {
            let content = fs::read_to_string(last_file).ok()?;
            let paths: Vec<PathBuf> = content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(PathBuf::from)
                .collect();
            if paths.is_empty() { None } else { Some(paths) }
        } else {
            None
        }
    }

    pub fn clear_all_metadata(&self) -> std::io::Result<()> {
        if self.metadata_dir.exists() {
            for entry in fs::read_dir(&self.metadata_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let _ = fs::remove_file(path);
                }
            }
        }
        Ok(())
    }

    pub fn clear(&self) -> std::io::Result<()> {
        self.clear_data_dir()?;
        self.clear_all_metadata()?;
        Ok(())
    }

    pub fn total_entries(&self) -> usize {
        let entries_dir = self.root.join(DATA_DIR_NAME);
        if let Ok(entries) = fs::read_dir(entries_dir) {
            entries.flatten().count()
        } else {
            1
        }
    }

    pub fn total_data_size(&self) -> u64 {
        let mut total = 0;
        if self.data_dir.exists() {
            for entry in walkdir::WalkDir::new(&self.data_dir).into_iter().flatten() {
                if entry.path().is_file()
                    && let Ok(meta) = entry.metadata()
                {
                    total += meta.len();
                }
            }
        }
        total
    }

    pub fn note(&self) -> Option<String> {
        let note_file = self.metadata_dir.join(NOTES_FILE_NAME);
        if note_file.is_file() {
            fs::read_to_string(note_file).ok()
        } else {
            None
        }
    }

    pub fn is_locked(&self) -> bool {
        let lock_file = self.metadata_dir.join(LOCK_FILE_NAME);
        lock_file.is_file()
    }
}
