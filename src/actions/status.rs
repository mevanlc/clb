use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde::Serialize;

use crate::config::Config;
use crate::gui::GuiClipboard;
use crate::storage::ClipboardStorage;
use crate::utils;

#[derive(Serialize)]
#[serde(untagged)]
enum StatusJsonItem {
    Text(String),
    Files(Vec<FileEntryJson>),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileEntryJson {
    filename: String,
    path: String,
    is_directory: bool,
}

pub fn execute(json_output: bool, config: &Config, gui: &GuiClipboard) -> Result<(), String> {
    // 1. Sync external/GUI clipboard into default clipboard if applicable
    sync_gui_clipboard(config, gui);

    // 2. Discover all clipboards with content across temp_dir and persist_dir
    let clipboards = find_clipboards_with_content(config);

    if json_output {
        let mut map = BTreeMap::new();
        for (name, storage) in &clipboards {
            if storage.holds_raw_data() {
                let text = storage.get_raw_text().unwrap_or_default();
                map.insert(name.clone(), StatusJsonItem::Text(text));
            } else if storage.holds_files() {
                let files = storage.get_files().unwrap_or_default();
                let file_entries = files
                    .into_iter()
                    .map(|f| FileEntryJson {
                        filename: f
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                        path: f.display().to_string(),
                        is_directory: f.is_dir(),
                    })
                    .collect();
                map.insert(name.clone(), StatusJsonItem::Files(file_entries));
            }
        }
        let out = serde_json::to_string_pretty(&map).map_err(|e| e.to_string())?;
        println!("{}", out);
        return Ok(());
    }

    if clipboards.is_empty() {
        println!("No clipboards have content.");
        println!("Use 'clb copy' or 'clb cut' to add items.");
        return Ok(());
    }

    let width: usize = 70;
    let title = " All your clipboards with content ";
    let bar_len = width.saturating_sub(title.len() + 5);
    let top_bar = format!("+---{}+{}", title, "-".repeat(bar_len));
    println!("{}", top_bar);

    let longest_name = clipboards.keys().map(|k| k.len()).max().unwrap_or(1);

    for (name, storage) in &clipboards {
        let preview = if storage.holds_raw_data() {
            let text = storage.get_raw_text().unwrap_or_default();
            let clean: String = text
                .chars()
                .map(|c| {
                    if c == '\n' || c == '\r' || c == '\t' {
                        ' '
                    } else {
                        c
                    }
                })
                .collect();
            if clean.len() > 50 {
                format!("{}...", &clean[..50])
            } else {
                clean
            }
        } else if storage.holds_files() {
            let files = storage.get_files().unwrap_or_default();
            let names: Vec<String> = files
                .iter()
                .map(|f| {
                    let n = f
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    if f.is_dir() { format!("{}/", n) } else { n }
                })
                .collect();
            names.join(", ")
        } else {
            "(empty)".to_string()
        };

        println!("| {:>width$} | {}", name, preview, width = longest_name);
    }

    let legend = " Legend: Text, Files, Directories, Data ";
    let bot_bar_len = width.saturating_sub(legend.len() + 5);
    let bottom_bar = format!("+---{}+{}", legend, "-".repeat(bot_bar_len));
    println!("{}", bottom_bar);

    Ok(())
}

fn sync_gui_clipboard(config: &Config, gui: &GuiClipboard) {
    if !config.gui_enabled {
        return;
    }
    if let Some(files) = gui.get_files()
        && !files.is_empty()
    {
        let storage = ClipboardStorage::new(config);
        let is_inside_storage = files.iter().all(|f| f.starts_with(&storage.data_dir));
        if !is_inside_storage {
            let _ = storage.clear_data_dir();
            for src in &files {
                if let Some(name) = src.file_name() {
                    let dst = storage.data_dir.join(name);
                    let _ = utils::copy_recursive(src, &dst, config.fast_copy);
                }
            }
        }
        return;
    }
    if let Some(gui_text) = gui.get_text()
        && !gui_text.is_empty()
    {
        let storage = ClipboardStorage::new(config);
        if !storage.holds_files() {
            let current = storage.get_raw_text();
            if current.as_deref() != Some(&gui_text) {
                let _ = storage.set_raw_text(&gui_text);
            }
        }
    }
}

fn find_clipboards_with_content(config: &Config) -> BTreeMap<String, ClipboardStorage> {
    let mut map = BTreeMap::new();

    // Check temporary directory
    scan_dir_for_clipboards(&config.temp_dir, config, &mut map);

    // Check persistent directory
    scan_dir_for_clipboards(&config.persist_dir, config, &mut map);

    map
}

fn scan_dir_for_clipboards(
    dir: &PathBuf,
    config: &Config,
    map: &mut BTreeMap<String, ClipboardStorage>,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let mut conf = config.clone();
            conf.clipboard_name = name.clone();
            let storage = ClipboardStorage::new(&conf);
            if storage.holds_data() && !map.contains_key(&name) {
                map.insert(name, storage);
            }
        }
    }
}
