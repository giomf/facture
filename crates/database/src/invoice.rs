use crate::schema::invoices;
use crate::Repository;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::invoices)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Invoice {
    pub id: i32,
    pub customer_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::invoices)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewInvoice {
    pub customer_id: i32,
}

#[derive(AsChangeset, Default, Debug)]
#[diesel(table_name = crate::schema::invoices)]
pub struct UpdateInvoice {
    pub customer_id: Option<i32>,
}
pub struct InvoiceRepository {
    connection: SqliteConnection,
}

impl Repository<Invoice, NewInvoice, UpdateInvoice> for InvoiceRepository {
    fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    fn create(&mut self, new_invoice: &NewInvoice) -> anyhow::Result<Invoice> {
        let new_invoice: Invoice = diesel::insert_into(invoices::table)
            .values(new_invoice)
            .get_result(&mut self.connection)?;
        Ok(new_invoice)
    }

    fn read(&mut self, id: i32) -> anyhow::Result<Option<Invoice>> {
        let invoice = invoices::dsl::invoices
            .find(id)
            .first::<Invoice>(&mut self.connection)
            .optional()?;

        Ok(invoice)
    }
    fn read_all(&mut self) -> anyhow::Result<Vec<Invoice>> {
        let invoices = invoices::table.get_results::<Invoice>(&mut self.connection)?;
        Ok(invoices)
    }

    fn update(&mut self, id: i32, update_invoice: &UpdateInvoice) -> anyhow::Result<Invoice> {
        let update_invoice: Invoice = diesel::update(invoices::table)
            .filter(invoices::dsl::id.eq(id))
            .set(update_invoice)
            .get_result(&mut self.connection)?;

        Ok(update_invoice)
    }

    fn delete(&mut self, id: i32) -> anyhow::Result<Invoice> {
        let deleted_invoice: Invoice = diesel::delete(invoices::table)
            .filter(invoices::dsl::id.eq(id))
            .get_result(&mut self.connection)?;

        Ok(deleted_invoice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_connection;
    use crate::customer::CustomerRepository;
    use crate::tests::*;
    use diesel_migrations::{FileBasedMigrations, MigrationHarness};
    use tempfile::{tempdir, TempDir};

    fn setup() -> anyhow::Result<(TempDir, CustomerRepository, InvoiceRepository)> {
        let temp_dir = tempdir()?;
        let database_path = temp_dir.path().join(DATABASE_NAME);
        let database_path = database_path.to_string_lossy();
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./migrations")?;
        let mut customers_connection = create_connection(&database_path);
        let invoices_connection = create_connection(&database_path);
        customers_connection
            .run_pending_migrations(migrations)
            .unwrap();
        let customers = CustomerRepository::new(customers_connection);
        let invoices = InvoiceRepository::new(invoices_connection);
        Ok((temp_dir, customers, invoices))
    }

    #[test]
    fn create() -> anyhow::Result<()> {
        let (_temp_dir, mut customers, mut invoices) = setup()?;
        let customer = customers.create(&NEW_CUSTOMER)?;
        let new_invoice = NewInvoice {
            customer_id: customer.id,
        };
        let invoice = invoices.create(&new_invoice)?;
        assert_eq!(customer.id, invoice.customer_id);
        Ok(())
    }

    #[test]
    fn read() -> anyhow::Result<()> {
        let (_temp_dir, mut customers, mut invoices) = setup()?;
        let customer = customers.create(&NEW_CUSTOMER)?;
        let new_invoice = NewInvoice {
            customer_id: customer.id,
        };
        let invoice = invoices.create(&new_invoice)?;
        let read_invoice = invoices.read(invoice.id)?;
        assert!(read_invoice.is_some());

        let read_invoice = read_invoice.unwrap();
        assert_eq!(invoice, read_invoice);
        Ok(())
    }

    #[test]
    fn read_all() -> anyhow::Result<()> {
        let (_temp_dir, mut customers, mut invoices) = setup()?;
        let customer = customers.create(&NEW_CUSTOMER)?;
        let new_invoice = NewInvoice {
            customer_id: customer.id,
        };
        let mut created_invoices = Vec::new();
        for _ in 0..2 {
            created_invoices.push(invoices.create(&new_invoice)?);
        }
        let read_invoices = invoices.read_all()?;
        assert_eq!(created_invoices, read_invoices);
        Ok(())
    }

    #[test]
    fn delete() -> anyhow::Result<()> {
        let (_temp_dir, mut customers, mut invoices) = setup()?;
        let customer = customers.create(&NEW_CUSTOMER)?;
        let new_invoice = NewInvoice {
            customer_id: customer.id,
        };
        let invoice = invoices.create(&new_invoice)?;
        let result = invoices.delete(invoice.id);
        assert!(result.is_ok());
        Ok(())
    }
}
