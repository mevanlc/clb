mod actions;
mod cli;
mod config;
mod gui;
mod storage;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use gui::GuiClipboard;
use storage::ClipboardStorage;

fn main() {
    let cli = Cli::parse();
    let config = Config::load(
        cli.clipboard.clone(),
        cli.entry,
        cli.fast_copy,
        cli.no_confirmation,
        cli.no_progress,
        cli.silent,
    );

    let storage = ClipboardStorage::new(&config);
    let gui = GuiClipboard::new(&config);

    let result = match cli.command {
        Some(Commands::Copy { items }) => actions::copy::execute(&items, &config, &storage, &gui),
        Some(Commands::Paste { patterns }) => {
            actions::paste::execute(&patterns, &config, &storage, &gui)
        }
        Some(Commands::Move { patterns }) => {
            actions::move_cmd::execute(&patterns, &config, &storage, &gui)
        }
        Some(Commands::Clear { entries, all }) => {
            actions::clear::execute(&entries, all, &config, &storage, &gui)
        }
        Some(Commands::Cut { items }) => actions::cut::execute(&items, &config, &storage, &gui),
        Some(Commands::Edit) => actions::edit::execute(&config, &storage, &gui),
        Some(Commands::Info) => actions::info::execute(cli.json, &config, &storage),
        Some(Commands::Status) => actions::info::execute(cli.json, &config, &storage),
        Some(Commands::Show { patterns }) => actions::show::execute(&patterns, &config, &storage),
        Some(Commands::Config) => actions::config::execute(cli.json, &config),
        Some(Commands::Completion { shell }) => actions::completion::execute(shell),

        // Deferred
        Some(Commands::Import { .. }) => actions::placeholders::execute("import", true),
        Some(Commands::Export { .. }) => actions::placeholders::execute("export", true),
        Some(Commands::History { .. }) => actions::placeholders::execute("history", true),
        Some(Commands::Ignore { .. }) => actions::placeholders::execute("ignore", true),
        Some(Commands::Search { .. }) => actions::placeholders::execute("search", true),

        // Not Planned
        Some(Commands::Undo) => actions::placeholders::execute("undo", false),
        Some(Commands::Redo) => actions::placeholders::execute("redo", false),
        Some(Commands::Swap { .. }) => actions::placeholders::execute("swap", false),
        Some(Commands::Load { .. }) => actions::placeholders::execute("load", false),
        Some(Commands::Remove { .. }) => actions::placeholders::execute("remove", false),
        Some(Commands::Add { .. }) => actions::placeholders::execute("add", false),
        Some(Commands::Note { .. }) => actions::placeholders::execute("note", false),
        Some(Commands::Script { .. }) => actions::placeholders::execute("script", false),
        Some(Commands::Share { .. }) => actions::placeholders::execute("share", false),

        None => {
            if !utils::is_stdin_terminal() {
                actions::copy::execute(&[], &config, &storage, &gui)
            } else if !utils::is_stdout_terminal() {
                actions::paste::execute(&[], &config, &storage, &gui)
            } else {
                actions::info::execute(cli.json, &config, &storage)
            }
        }
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
