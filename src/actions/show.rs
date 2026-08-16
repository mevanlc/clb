use regex::Regex;

use crate::config::Config;
use crate::storage::ClipboardStorage;

pub fn execute(
    patterns: &[String],
    _config: &Config,
    storage: &ClipboardStorage,
) -> Result<(), String> {
    storage.ensure_dirs().map_err(|e| e.to_string())?;

    let regexes: Result<Vec<Regex>, regex::Error> =
        patterns.iter().map(|p| Regex::new(p)).collect();
    let regexes = regexes.map_err(|e| format!("Invalid regex pattern: {}", e))?;

    if storage.holds_raw_data() {
        if let Some(text) = storage.get_raw_text() {
            print!("{}", text);
            if !text.ends_with('\n') {
                println!();
            }
        }
    } else if storage.holds_files() {
        let files = storage.get_files().map_err(|e| e.to_string())?;
        for file in files {
            let name = file
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            if !regexes.is_empty() && !regexes.iter().any(|r| r.is_match(&name)) {
                continue;
            }
            if file.is_dir() {
                println!("{}/", name);
            } else {
                println!("{}", name);
            }
        }
    }

    Ok(())
}
