mod commands;

use clap::Parser;
use commands::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli {
        Cli::Customer(cmd) => cmd.execute()?,
        Cli::Invoice(cmd) => cmd.execute()?,
    }
    Ok(())
}
