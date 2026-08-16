use clap::CommandFactory;
use clap_complete::{Shell, generate};

use crate::cli::Cli;

pub fn execute(shell: Shell) -> Result<(), String> {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "clb", &mut std::io::stdout());
    Ok(())
}
