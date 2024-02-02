use crate::schema::customers;
use crate::Repository;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable, Selectable)]
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

struct Customers {
    connection: SqliteConnection,
}

impl Repository<Customer, NewCustomer> for Customers {
    fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    fn read(&mut self, id: i32) -> anyhow::Result<Option<Customer>> {
        let customer = customers::dsl::customers
            .find(id)
            .first::<Customer>(&mut self.connection)
            .optional()?;

        Ok(customer)
    }

    fn create(&mut self, element: &NewCustomer) -> anyhow::Result<()> {
        diesel::insert_into(customers::table)
            .values(element)
            .execute(&mut self.connection)?;

        Ok(())
    }

    fn delete(&mut self, id: i32) -> anyhow::Result<()> {
        diesel::delete(customers::table)
            .filter(customers::dsl::id.eq(id))
            .execute(&mut self.connection)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    fn setup() -> anyhow::Result<(TempDir, Customers)> {
        let temp_dir = tempdir()?;
        let database_path = temp_dir.path().join(DATABASE_NAME);
        let database_path = database_path.to_string_lossy();
        let mut connection = SqliteConnection::establish(&database_path)
            .expect(format!("Unable to open {}", database_path).as_str());
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./migrations")?;
        connection.run_pending_migrations(migrations).unwrap();
        let customers = Customers::new(connection);
        Ok((temp_dir, customers))
    }

    #[test]
    fn create() {
        let (_temp_dir, mut customers) = setup().unwrap();
        let result = customers.create(&NEW_CUSTOMER);
        assert!(result.is_ok());
    }
}
