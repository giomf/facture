use std::path::Path;

use super::Command;
use crate::database::{
    create_connection,
    item::{ItemRepository, NewItem, UpdateItem},
    Repository, DATABASE_PATH,
};
use clap::{Args, Subcommand};

#[derive(Args, Clone, Debug)]
pub struct CreateArgs {
    /// Item position
    #[arg(long)]
    pub position: i32,

    /// Item name
    #[arg(long)]
    pub name: String,

    /// Item amount
    #[arg(long)]
    pub amount: i32,

    /// Item price
    #[arg(long)]
    pub price: f32,

    /// Invoice id
    #[arg(long)]
    pub invoice_id: i32,
}

#[derive(Args, Clone, Debug)]
pub struct DeleteArgs {
    /// Item ID
    pub id: i32,
}

#[derive(Args, Clone, Debug)]
pub struct EditArgs {
    /// Item ID
    pub id: i32,

    /// Item position
    #[arg(long)]
    pub position: Option<i32>,

    /// Item name
    #[arg(long)]
    pub name: Option<String>,

    /// Item amount
    #[arg(long)]
    pub amount: Option<i32>,

    /// Item price
    #[arg(long)]
    pub price: Option<f32>,
}

#[derive(Debug, Subcommand)]
pub enum ItemCommand {
    /// Create a new customer
    Create(CreateArgs),

    /// Delete a customer
    Delete(DeleteArgs),

    /// List all customers
    List,

    /// Edit a customer
    Edit(EditArgs),
}

impl Command for ItemCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let connection = create_connection(Path::new(DATABASE_PATH));
        let customers = ItemRepository::new(connection);
        match &self {
            ItemCommand::Create(args) => create(customers, args.clone()),
            ItemCommand::Delete(args) => delete(customers, args.id),
            ItemCommand::List => list(customers),
            ItemCommand::Edit(args) => edit(customers, args.clone()),
        }
    }
}

fn create(mut items: ItemRepository, args: CreateArgs) -> anyhow::Result<()> {
    let new_item = NewItem {
        name: args.name,
        position: args.position,
        amount: args.amount,
        price: args.price,
        invoice_id: args.invoice_id,
    };

    let new_item = items.create(&new_item)?;
    println!("{:?}", new_item);
    Ok(())
}

fn delete(mut items: ItemRepository, item_id: i32) -> anyhow::Result<()> {
    let deleted_item = items.delete(item_id)?;
    println!("{:?}", deleted_item);
    Ok(())
}

fn list(mut items: ItemRepository) -> anyhow::Result<()> {
    for item in items.read_all()? {
        println!("{:?}", item);
    }
    Ok(())
}

fn edit(mut items: ItemRepository, args: EditArgs) -> anyhow::Result<()> {
    let item_update = UpdateItem {
        name: args.name,
        position: args.position,
        amount: args.amount,
        price: args.price,
    };

    let updated_item = items.update(args.id, &item_update)?;
    println!("{:?}", updated_item);
    Ok(())
}
