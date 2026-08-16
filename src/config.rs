use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub clipboard_name: String,
    pub clipboard_entry: usize,
    pub temp_dir: PathBuf,
    pub persist_dir: PathBuf,
    pub editor: Option<String>,
    pub gui_enabled: bool,
    pub silent: bool,
    pub no_progress: bool,
    pub no_confirmation: bool,
    pub fast_copy: bool,
    pub ci_mode: bool,
    pub no_color: bool,
}

impl Config {
    pub fn load(
        clipboard_name: Option<String>,
        clipboard_entry: Option<usize>,
        fast_copy: bool,
        no_confirmation: bool,
        no_progress: bool,
        silent: bool,
    ) -> Self {
        let name = clipboard_name
            .unwrap_or_else(|| std::env::var("CLIPBOARD_NAME").unwrap_or_else(|_| "1".to_string()));
        let entry = clipboard_entry.unwrap_or(0);

        let temp_dir = if let Ok(val) = std::env::var("CLIPBOARD_TMPDIR") {
            PathBuf::from(val)
        } else if let Ok(val) = std::env::var("XDG_RUNTIME_DIR") {
            PathBuf::from(val).join("Clipboard")
        } else {
            std::env::temp_dir().join("Clipboard")
        };

        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());

        let persist_dir = if let Ok(val) = std::env::var("CLIPBOARD_PERSISTDIR") {
            PathBuf::from(val)
        } else if let Ok(val) = std::env::var("XDG_STATE_HOME") {
            PathBuf::from(val).join("clipboard")
        } else {
            PathBuf::from(home).join(".local/state/clipboard")
        };

        let editor = std::env::var("CLIPBOARD_EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .or_else(|_| std::env::var("EDITOR"))
            .ok();

        let gui_enabled = !is_env_true("CLIPBOARD_NOGUI");
        let silent_env = is_env_true("CLIPBOARD_SILENT");
        let progress_env = is_env_true("CLIPBOARD_NOPROGRESS");
        let ci_mode = std::env::var("CI").is_ok();
        let no_color = std::env::var("NO_COLOR").is_ok();

        Self {
            clipboard_name: name,
            clipboard_entry: entry,
            temp_dir,
            persist_dir,
            editor,
            gui_enabled,
            silent: silent || silent_env,
            no_progress: no_progress || progress_env,
            no_confirmation,
            fast_copy,
            ci_mode,
            no_color,
        }
    }
}

fn is_env_true(key: &str) -> bool {
    match std::env::var(key) {
        Ok(v) => {
            let v = v.trim().to_lowercase();
            v == "1" || v == "true" || v == "yes"
        }
        Err(_) => false,
    }
}
