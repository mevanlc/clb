use serde::Serialize;

use crate::config::Config;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ConfigJson {
    editor: Option<String>,
    temporary_directory: String,
    persistent_directory: String,
    gui_integration: bool,
    silent_mode: bool,
    progress_silent: bool,
    no_confirmation: bool,
    fast_copy: bool,
    ci_mode: bool,
    color_output: bool,
}

pub fn execute(json_output: bool, config: &Config) -> Result<(), String> {
    if json_output {
        let conf = ConfigJson {
            editor: config.editor.clone(),
            temporary_directory: config.temp_dir.display().to_string(),
            persistent_directory: config.persist_dir.display().to_string(),
            gui_integration: config.gui_enabled,
            silent_mode: config.silent,
            progress_silent: config.no_progress,
            no_confirmation: config.no_confirmation,
            fast_copy: config.fast_copy,
            ci_mode: config.ci_mode,
            color_output: !config.no_color,
        };
        let out = serde_json::to_string_pretty(&conf).map_err(|e| e.to_string())?;
        println!("{}", out);
        return Ok(());
    }

    let width: usize = 67;
    let title = " Configuration ";
    let bar_len = width.saturating_sub(title.len() + 5);
    let top_bar = format!("+---{}+{}", title, "-".repeat(bar_len));

    println!("{}", top_bar);
    println!(
        "| Content editor:        {}",
        config.editor.as_deref().unwrap_or("None")
    );
    println!("| Temporary directory:   {}", config.temp_dir.display());
    println!("| Persistent directory:  {}", config.persist_dir.display());
    println!(
        "| GUI integration:       {}",
        if config.gui_enabled {
            "enabled"
        } else {
            "disabled"
        }
    );
    println!(
        "| Silent output:         {}",
        if config.silent { "enabled" } else { "disabled" }
    );
    println!(
        "| Fast copy:             {}",
        if config.fast_copy {
            "enabled"
        } else {
            "disabled"
        }
    );
    println!("| Terminal styling:      7-bit ascii");

    let bottom_bar = format!("+{}", "-".repeat(width.saturating_sub(1)));
    println!("{}", bottom_bar);

    Ok(())
}
