pub mod customer;
use self::customer::CustomerCommand;
use clap::Parser;

pub trait Command {
    fn execute(&self) -> anyhow::Result<()>;
}

#[derive(Parser)]
#[clap(author, version, about = "facture", long_about = None)]
#[clap(propagate_version = true)]
pub enum Cli {
    #[command(subcommand)]
    Customer(CustomerCommand),
}
