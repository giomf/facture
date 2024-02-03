mod commands;

use clap::Parser;
use commands::Command;

fn main() -> anyhow::Result<()> {
    let cli = commands::Cli::parse();
    match &cli {
        commands::Cli::Customer(cmd) => cmd.execute()?,
    }
    Ok(())
}
