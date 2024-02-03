use crate::schema::customers;
use crate::Repository;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCustomer {
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

pub struct CustomerRepository {
    connection: SqliteConnection,
}

impl Repository<Customer, NewCustomer> for CustomerRepository {
    fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    fn create(&mut self, new_customer: &NewCustomer) -> anyhow::Result<Customer> {
        let new_customer: Customer = diesel::insert_into(customers::table)
            .values(new_customer)
            .get_result(&mut self.connection)?;
        Ok(new_customer)
    }

    fn read(&mut self, id: i32) -> anyhow::Result<Option<Customer>> {
        let customer = customers::dsl::customers
            .find(id)
            .first::<Customer>(&mut self.connection)
            .optional()?;

        Ok(customer)
    }
    fn read_all(&mut self) -> anyhow::Result<Vec<Customer>> {
        let cusomers = customers::table.get_results::<Customer>(&mut self.connection)?;
        Ok(cusomers)
    }

    fn update(&mut self) -> anyhow::Result<Customer> {
        todo!()
    }

    fn delete(&mut self, id: i32) -> anyhow::Result<Customer> {
        let deleted_customer: Customer = diesel::delete(customers::table)
            .filter(customers::dsl::id.eq(id))
            .get_result(&mut self.connection)?;

        Ok(deleted_customer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_connection;
    use diesel_migrations::{FileBasedMigrations, MigrationHarness};
    use lazy_static::lazy_static;
    use tempfile::{tempdir, TempDir};

    const DATABASE_NAME: &str = "facture_test.db";
    const NAME: &str = "Jon";
    const SURNAME: &str = "Doe";
    const EMAIL: &str = "jon.doe@example.com";
    const PHONE: &str = "0123456789";

    lazy_static! {
        static ref NEW_CUSTOMER: NewCustomer = NewCustomer {
            name: NAME.to_string(),
            surname: SURNAME.to_string(),
            email: Some(EMAIL.to_string()),
            phone: Some(PHONE.to_string())
        };
    }

    fn setup() -> anyhow::Result<(TempDir, CustomerRepository)> {
        let temp_dir = tempdir()?;
        let database_path = temp_dir.path().join(DATABASE_NAME);
        let database_path = database_path.to_string_lossy();
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./migrations")?;
        let mut connection = create_connection(&database_path);
        connection.run_pending_migrations(migrations).unwrap();
        let customers = CustomerRepository::new(connection);
        Ok((temp_dir, customers))
    }

    #[test]
    fn create() {
        let (_temp_dir, mut customers) = setup().unwrap();
        let result = customers.create(&NEW_CUSTOMER);

        assert!(result.is_ok());
    }

    #[test]
    fn read() -> anyhow::Result<()> {
        let (_temp_dir, mut customers) = setup().unwrap();
        let created_customer = customers.create(&NEW_CUSTOMER)?;
        let readed_customer = customers.read(created_customer.id)?.unwrap();

        assert_eq!(created_customer, readed_customer);
        Ok(())
    }

    #[test]
    fn read_all() -> anyhow::Result<()> {
        let (_temp_dir, mut customers) = setup().unwrap();
        let created_customer_0 = customers.create(&NEW_CUSTOMER)?;
        let created_customer_1 = customers.create(&NEW_CUSTOMER)?;
        let created_customer_2 = customers.create(&NEW_CUSTOMER)?;
        let readed_customers = customers.read_all()?;
        let created_customers = vec![created_customer_0, created_customer_1, created_customer_2];

        assert_eq!(created_customers, readed_customers);
        Ok(())
    }

    #[test]
    fn delete() -> anyhow::Result<()> {
        let (_temp_dir, mut customers) = setup().unwrap();
        let customer = customers.create(&NEW_CUSTOMER)?;
        let result = customers.delete(customer.id);

        assert!(result.is_ok());
        Ok(())
    }
}
