pub mod customer;
pub mod invoice;
pub mod items;
mod schema;

use std::path::Path;

use diesel::{sqlite::SqliteConnection, Connection};

pub const DATABASE_PATH: &str = "./facture.sqlite";

pub trait Repository<T, E, U> {
    fn new(connection: SqliteConnection) -> Self;
    fn exists(&mut self, id: i32) -> anyhow::Result<bool>;
    fn create(&mut self, element: &E) -> anyhow::Result<T>;
    fn read(&mut self, id: i32) -> anyhow::Result<Option<T>>;
    fn read_all(&mut self) -> anyhow::Result<Vec<T>>;
    fn update(&mut self, id: i32, element: &U) -> anyhow::Result<T>;
    fn delete(&mut self, id: i32) -> anyhow::Result<T>;
}

pub fn create_connection(database_path: &Path) -> SqliteConnection {
    let database_path = database_path.to_string_lossy();
    SqliteConnection::establish(&database_path)
        .expect(format!("Unable to open {}", database_path).as_str())
}

#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use crate::database::invoice::NewInvoice;

    use super::{
        create_connection,
        customer::{Customer, CustomerRepository, NewCustomer},
        invoice::{Invoice, InvoiceRepository},
        items::{Item, ItemRepository, NewItem},
        Repository,
    };
    use diesel_migrations::{FileBasedMigrations, MigrationHarness};
    use lazy_static::lazy_static;
    use tempfile::{tempdir, TempDir};

    pub const DATABASE_NAME: &str = "facture_test.db";
    pub const CUSTOMER_NAME: &str = "Jon";
    pub const CUSTOMER_SURNAME: &str = "Doe";
    pub const CUSTOMER_EMAIL: &str = "jon.doe@example.com";
    pub const CUSTOMER_PHONE: &str = "0123456789";

    pub const ITEM_POSITION: i32 = 1;
    pub const ITEM_NAME: &str = "Item";
    pub const ITEM_AMOUNT: i32 = 1;
    pub const ITEM_PRICE: f32 = 10.0;

    lazy_static! {
        pub static ref NEW_CUSTOMER: NewCustomer = NewCustomer {
            name: CUSTOMER_NAME.to_string(),
            surname: CUSTOMER_SURNAME.to_string(),
            email: Some(CUSTOMER_EMAIL.to_string()),
            phone: Some(CUSTOMER_PHONE.to_string())
        };
    }

    pub fn init_database() -> anyhow::Result<(TempDir, PathBuf)> {
        let temp_dir = tempdir()?;
        let database_path = temp_dir.path().join(DATABASE_NAME);
        let migrations =
            FileBasedMigrations::find_migrations_directory_in_path("src/database/migrations")?;
        let mut migration_connection = create_connection(&database_path);
        migration_connection
            .run_pending_migrations(migrations)
            .expect("Unable to migrate database");
        Ok((temp_dir, database_path))
    }

    pub fn init_customer(customers: &mut CustomerRepository) -> anyhow::Result<Customer> {
        Ok(customers.create(&NEW_CUSTOMER)?)
    }

    pub fn init_invoice(
        customer: &Customer,
        invoices: &mut InvoiceRepository,
    ) -> anyhow::Result<Invoice> {
        let new_invoice = NewInvoice {
            customer_id: customer.id,
        };

        let invoice = invoices.create(&new_invoice)?;
        Ok(invoice)
    }

    pub fn init_item(invoice: &Invoice, items: &mut ItemRepository) -> anyhow::Result<Item> {
        let new_item = NewItem {
            position: ITEM_POSITION,
            name: ITEM_NAME.to_string(),
            amount: ITEM_AMOUNT,
            price: ITEM_PRICE,
            invoice_id: invoice.id,
        };
        let item = items.create(&new_item)?;
        Ok(item)
    }
}
