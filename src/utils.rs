use std::fs;
use std::io::IsTerminal;
use std::path::Path;

pub fn is_stdin_terminal() -> bool {
    std::io::stdin().is_terminal()
}

pub fn is_stdout_terminal() -> bool {
    std::io::stdout().is_terminal()
}

pub fn copy_recursive(src: &Path, dst: &Path, fast_copy: bool) -> std::io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let child_src = entry.path();
            let child_dst = dst.join(entry.file_name());
            copy_recursive(&child_src, &child_dst, fast_copy)?;
        }
    } else {
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }
        if fast_copy {
            // Try hard link first, fallback to copy
            if fs::hard_link(src, dst).is_err() {
                fs::copy(src, dst)?;
            }
        } else {
            fs::copy(src, dst)?;
        }
    }
    Ok(())
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;
    if b < KB {
        format!("{}B", bytes)
    } else if b < MB {
        format!("{:.2}KB", b / KB)
    } else if b < GB {
        format!("{:.2}MB", b / MB)
    } else {
        format!("{:.2}GB", b / GB)
    }
}

pub fn infer_mime_type(content: &str) -> &'static str {
    let trimmed = content.trim_start();
    let is_json_shaped = (trimmed.starts_with('{') && trimmed.ends_with('}'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']'));
    if is_json_shaped && serde_json::from_str::<serde_json::Value>(content).is_ok() {
        return "application/json";
    }
    if trimmed.starts_with("<!DOCTYPE html")
        || trimmed.starts_with("<html")
        || trimmed.starts_with("<?xml")
    {
        return "text/html";
    }
    "text/plain"
}
