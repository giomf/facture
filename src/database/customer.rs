use crate::database::{schema::customers, Repository};
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = customers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = customers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCustomer {
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(AsChangeset, Default, Debug)]
#[diesel(table_name = customers)]
pub struct UpdateCustomer {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

pub struct CustomerRepository {
    connection: SqliteConnection,
}

impl Repository<Customer, NewCustomer, UpdateCustomer> for CustomerRepository {
    fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    fn exists(&mut self, customer_id: i32) -> anyhow::Result<bool> {
        use crate::database::schema::customers::dsl::*;
        let result = select(exists(customers.filter(id.eq(customer_id))))
            .get_result(&mut self.connection)?;

        Ok(result)
    }

    fn create(&mut self, new_customer: &NewCustomer) -> anyhow::Result<Customer> {
        use crate::database::schema::customers::dsl::*;
        let new_customer: Customer = diesel::insert_into(customers)
            .values(new_customer)
            .get_result(&mut self.connection)?;
        Ok(new_customer)
    }

    fn read(&mut self, customer_id: i32) -> anyhow::Result<Option<Customer>> {
        use crate::database::schema::customers::dsl::*;
        let customer = customers
            .find(customer_id)
            .first::<Customer>(&mut self.connection)
            .optional()?;

        Ok(customer)
    }
    fn read_all(&mut self) -> anyhow::Result<Vec<Customer>> {
        use crate::database::schema::customers::dsl::*;
        let all_customers = customers.get_results::<Customer>(&mut self.connection)?;
        Ok(all_customers)
    }

    fn update(
        &mut self,
        customer_id: i32,
        update_customer: &UpdateCustomer,
    ) -> anyhow::Result<Customer> {
        use crate::database::schema::customers::dsl::*;
        let update_customer: Customer = diesel::update(customers)
            .filter(id.eq(customer_id))
            .set(update_customer)
            .get_result(&mut self.connection)?;

        Ok(update_customer)
    }

    fn delete(&mut self, customer_id: i32) -> anyhow::Result<Customer> {
        use crate::database::schema::customers::dsl::*;
        let deleted_customer: Customer = diesel::delete(customers)
            .filter(id.eq(customer_id))
            .get_result(&mut self.connection)?;

        Ok(deleted_customer)
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::database::{create_connection, tests::*};
    use tempfile::TempDir;

    fn setup() -> anyhow::Result<(TempDir, CustomerRepository)> {
        let (temp_dir, database_path) = init_database()?;
        let connection = create_connection(&database_path);
        let customers = CustomerRepository::new(connection);
        Ok((temp_dir, customers))
    }

    #[test]
    fn create() -> anyhow::Result<()> {
        let (_temp_dir, mut customers) = setup()?;
        let result = customers.create(&NEW_CUSTOMER);

        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn read() -> anyhow::Result<()> {
        let (_temp_dir, mut customers) = setup()?;
        let created_customer = customers.create(&NEW_CUSTOMER)?;
        let read_customer = customers.read(created_customer.id)?;
        assert!(read_customer.is_some());
        let read_customer = read_customer.unwrap();

        assert_eq!(created_customer, read_customer);
        Ok(())
    }

    #[test]
    fn read_all() -> anyhow::Result<()> {
        let (_temp_dir, mut customers) = setup()?;
        let created_customer_0 = customers.create(&NEW_CUSTOMER)?;
        let created_customer_1 = customers.create(&NEW_CUSTOMER)?;
        let created_customer_2 = customers.create(&NEW_CUSTOMER)?;
        let readed_customers = customers.read_all()?;
        let created_customers = vec![created_customer_0, created_customer_1, created_customer_2];

        assert_eq!(created_customers, readed_customers);
        Ok(())
    }

    #[test]
    fn update() -> anyhow::Result<()> {
        let (_temp_dir, mut customers) = setup().unwrap();
        let created_customer = customers.create(&NEW_CUSTOMER)?;
        let updated_name = "John".to_string();
        let update_customer = UpdateCustomer {
            name: Some(updated_name.clone()),
            ..Default::default()
        };

        let updated_customer = customers.update(created_customer.id, &update_customer)?;
        let test_result = Customer {
            name: updated_name,
            ..created_customer
        };

        assert_eq!(test_result, updated_customer);
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
