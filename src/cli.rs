use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(
    name = "clb",
    about = "clb - Rust port/reimplementation of a subset of Clipboard (cb)",
    version,
    subcommand_precedence_over_arg = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Choose which clipboard to use
    #[arg(short = 'c', long = "clipboard", global = true)]
    pub clipboard: Option<String>,

    /// Choose which history entry to use
    #[arg(short = 'e', long = "entry", global = true)]
    pub entry: Option<usize>,

    /// Use links when copying, cutting, or pasting
    #[arg(long = "fast-copy", short_alias = 'f', global = true)]
    pub fast_copy: bool,

    /// Disable confirmation messages
    #[arg(long = "no-confirmation", global = true)]
    pub no_confirmation: bool,

    /// Disable progress messages
    #[arg(long = "no-progress", global = true)]
    pub no_progress: bool,

    /// Disable progress and confirmation messages
    #[arg(short = 's', long = "silent", global = true)]
    pub silent: bool,

    /// Output in JSON format (for info, status, config)
    #[arg(long = "json", global = true)]
    pub json: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Copy items into a clipboard
    #[command(alias = "cp", alias = "c")]
    Copy {
        /// Files, directories, or text to copy
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        items: Vec<String>,
    },

    /// Paste items from a clipboard
    #[command(alias = "p")]
    Paste {
        /// Optional regex patterns to filter pasted items
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        patterns: Vec<String>,
    },

    /// Treat the last copy as a cut, then paste
    #[command(name = "move", alias = "mv")]
    Move {
        /// Optional regex patterns to filter moved items
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        patterns: Vec<String>,
    },

    /// Clear a clipboard
    #[command(alias = "clr")]
    Clear {
        /// Entries to clear
        entries: Vec<String>,

        /// Clear all clipboards
        #[arg(short = 'a', long = "all")]
        all: bool,
    },

    /// Cut items into a clipboard
    #[command(alias = "ct")]
    Cut {
        /// Files, directories, or text to cut
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        items: Vec<String>,
    },

    /// Edit the contents of a clipboard as text
    #[command(alias = "ed")]
    Edit,

    /// Show info about a clipboard
    #[command(alias = "in")]
    Info,

    /// Check the status of a clipboard
    #[command(alias = "st")]
    Status,

    /// Show the contents of a clipboard
    #[command(alias = "sh")]
    Show {
        /// Optional regex patterns to filter shown items
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        patterns: Vec<String>,
    },

    /// Show the configuration of clb
    #[command(alias = "cfg")]
    Config,

    /// Generate shell completion scripts for clb
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    // Deferred / Placeholders
    /// Import a clipboard from a file (Deferred)
    #[command(alias = "imp")]
    Import {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Export a clipboard to a file (Deferred)
    #[command(alias = "ex")]
    Export {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Show the history of a clipboard (Deferred)
    #[command(alias = "hs")]
    History {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Ignore types of content in a clipboard (Deferred)
    #[command(alias = "ig")]
    Ignore {
        #[arg(trailing_var_arg = true)]
        patterns: Vec<String>,
    },

    /// Search for items in a clipboard (Deferred)
    #[command(alias = "sr")]
    Search {
        #[arg(trailing_var_arg = true)]
        query: Vec<String>,
    },

    /// Undo clipboard action (Not implemented)
    #[command(alias = "u")]
    Undo,

    /// Redo clipboard action (Not implemented)
    #[command(alias = "r")]
    Redo,

    /// Swap the contents of multiple clipboards (Not implemented)
    #[command(alias = "sw")]
    Swap {
        #[arg(trailing_var_arg = true)]
        clipboards: Vec<String>,
    },

    /// Load a clipboard into another clipboard (Not implemented)
    #[command(alias = "ld")]
    Load {
        #[arg(trailing_var_arg = true)]
        clipboards: Vec<String>,
    },

    /// Remove items from a clipboard (Not implemented)
    #[command(alias = "rm")]
    Remove {
        #[arg(trailing_var_arg = true)]
        patterns: Vec<String>,
    },

    /// Add items to a clipboard (Not implemented)
    #[command(alias = "ad")]
    Add {
        #[arg(trailing_var_arg = true)]
        items: Vec<String>,
    },

    /// Add a note to a clipboard (Not implemented)
    #[command(alias = "nt")]
    Note {
        #[arg(trailing_var_arg = true)]
        text: Vec<String>,
    },

    /// Set a script to run for this clipboard (Not implemented)
    #[command(alias = "sc")]
    Script {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Share a clipboard with others (Not implemented)
    #[command(alias = "shr")]
    Share {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}
