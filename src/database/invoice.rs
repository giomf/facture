use crate::database::{schema::invoices, Repository};
use diesel::sqlite::SqliteConnection;
use diesel::{dsl::exists, prelude::*, select};

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = invoices)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Invoice {
    pub id: i32,
    pub customer_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = invoices)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewInvoice {
    pub customer_id: i32,
}

#[derive(AsChangeset, Default, Debug)]
#[diesel(table_name = invoices)]
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
        use crate::database::schema::invoices::dsl::*;

        let new_invoice: Invoice = diesel::insert_into(invoices)
            .values(new_invoice)
            .get_result(&mut self.connection)?;

        Ok(new_invoice)
    }

    fn exists(&mut self, invoice_id: i32) -> anyhow::Result<bool> {
        use crate::database::schema::invoices::dsl::*;

        let result =
            select(exists(invoices.filter(id.eq(invoice_id)))).get_result(&mut self.connection)?;

        Ok(result)
    }

    fn read(&mut self, invoice_id: i32) -> anyhow::Result<Option<Invoice>> {
        use crate::database::schema::invoices::dsl::*;

        let invoice = invoices
            .find(invoice_id)
            .first::<Invoice>(&mut self.connection)
            .optional()?;

        Ok(invoice)
    }

    fn read_all(&mut self) -> anyhow::Result<Vec<Invoice>> {
        use crate::database::schema::invoices::dsl::*;

        let all_invoices = invoices.get_results::<Invoice>(&mut self.connection)?;

        Ok(all_invoices)
    }

    fn update(
        &mut self,
        invoice_id: i32,
        update_invoice: &UpdateInvoice,
    ) -> anyhow::Result<Invoice> {
        use crate::database::schema::invoices::dsl::*;

        let update_invoice: Invoice = diesel::update(invoices)
            .filter(id.eq(invoice_id))
            .set(update_invoice)
            .get_result(&mut self.connection)?;

        Ok(update_invoice)
    }

    fn delete(&mut self, invoice_id: i32) -> anyhow::Result<Invoice> {
        use crate::database::schema::invoices::dsl::*;

        let deleted_invoice: Invoice = diesel::delete(invoices)
            .filter(id.eq(invoice_id))
            .get_result(&mut self.connection)?;

        Ok(deleted_invoice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{create_connection, customer::CustomerRepository, tests::*};
    use diesel_migrations::{FileBasedMigrations, MigrationHarness};
    use tempfile::{tempdir, TempDir};

    fn setup() -> anyhow::Result<(TempDir, CustomerRepository, InvoiceRepository)> {
        let temp_dir = tempdir()?;
        let database_path = temp_dir.path().join(DATABASE_NAME);
        let database_path = database_path.to_string_lossy();
        let migrations =
            FileBasedMigrations::find_migrations_directory_in_path("src/database/migrations")?;
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
