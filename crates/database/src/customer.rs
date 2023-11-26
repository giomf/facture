use crate::schema::customers;
use crate::{Repository, DATABASE_URL};
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
    fn new() -> Self {
        Self {
            connection: SqliteConnection::establish(DATABASE_URL)
                .expect(format!("Unable to open {DATABASE_URL}").as_str()),
        }
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
