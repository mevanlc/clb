use serde::Serialize;

use crate::config::Config;
use crate::storage::ClipboardStorage;
use crate::utils;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InfoJson {
    name: String,
    entry: usize,
    path: String,
    is_persistent: bool,
    total_entries: usize,
    total_bytes_used: u64,
    content_bytes: u64,
    content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directories: Option<usize>,
    content_cut: bool,
    locked: bool,
    note: Option<String>,
}

pub fn execute(
    json_output: bool,
    _config: &Config,
    storage: &ClipboardStorage,
) -> Result<(), String> {
    storage.ensure_dirs().map_err(|e| e.to_string())?;

    let is_persistent = storage.is_persistent;
    let total_entries = storage.total_entries();
    let total_size = storage.total_data_size();
    let has_cut = storage.get_originals().is_some();
    let is_locked = storage.is_locked();
    let note = storage.note();

    let (content_type, content_bytes, file_count, dir_count, text_preview) =
        if storage.holds_raw_data() {
            let text = storage.get_raw_text().unwrap_or_default();
            let mime = utils::infer_mime_type(&text).to_string();
            let preview = if text.len() > 60 {
                format!("{}...", &text[..60])
            } else {
                text.clone()
            };
            (mime, text.len() as u64, None, None, Some(preview))
        } else if storage.holds_files() {
            let files = storage.get_files().unwrap_or_default();
            let mut f_cnt = 0;
            let mut d_cnt = 0;
            for f in &files {
                if f.is_dir() {
                    d_cnt += 1;
                } else {
                    f_cnt += 1;
                }
            }
            let type_str = format!("{} file(s), {} dir(s)", f_cnt, d_cnt);
            (type_str, total_size, Some(f_cnt), Some(d_cnt), None)
        } else {
            ("empty".to_string(), 0, None, None, None)
        };

    if json_output {
        let info = InfoJson {
            name: storage.name.clone(),
            entry: storage.entry,
            path: storage.root.display().to_string(),
            is_persistent,
            total_entries,
            total_bytes_used: total_size,
            content_bytes,
            content_type,
            files: file_count,
            directories: dir_count,
            content_cut: has_cut,
            locked: is_locked,
            note,
        };
        let out = serde_json::to_string_pretty(&info).map_err(|e| e.to_string())?;
        println!("{}", out);
        return Ok(());
    }

    let separator = "-".repeat(67);

    println!("Clipboard {} (Entry {})", storage.name, storage.entry);
    println!("{}", separator);
    println!("Stored in:             {}", storage.root.display());
    println!(
        "Persistent:            {}",
        if is_persistent { "Yes" } else { "No" }
    );
    println!("Total entries:         {}", total_entries);
    println!("Total clipboard size:  {}", utils::format_bytes(total_size));
    println!(
        "Content size:          {}",
        utils::format_bytes(content_bytes)
    );
    println!("Content type:          {}", content_type);

    if let Some(preview) = text_preview {
        // Strip non-printable ascii
        let clean_preview: String = preview
            .chars()
            .map(|c| {
                if c == '\n' || c == '\r' || c == '\t' {
                    ' '
                } else {
                    c
                }
            })
            .collect();
        println!("Content preview:       {}", clean_preview);
    }

    if let Some(fc) = file_count {
        println!("Files:                 {}", fc);
    }
    if let Some(dc) = dir_count {
        println!("Directories:           {}", dc);
    }

    println!(
        "Content cut:           {}",
        if has_cut { "Yes" } else { "No" }
    );
    println!(
        "Locked:                {}",
        if is_locked { "Yes" } else { "No" }
    );

    if let Some(n) = note {
        println!("Note:                  {}", n.trim());
    }

    println!("{}", separator);

    Ok(())
}
