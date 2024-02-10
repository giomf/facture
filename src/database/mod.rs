pub mod customer;
pub mod invoice;
mod schema;

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

pub fn create_connection(database_path: &str) -> SqliteConnection {
    SqliteConnection::establish(database_path)
        .expect(format!("Unable to open {}", database_path).as_str())
}

#[cfg(test)]
pub mod tests {
    use crate::database::customer::NewCustomer;
    use lazy_static::lazy_static;

    pub const DATABASE_NAME: &str = "facture_test.db";
    pub const CUSTOMER_NAME: &str = "Jon";
    pub const CUSTOMER_SURNAME: &str = "Doe";
    pub const CUSTOMER_EMAIL: &str = "jon.doe@example.com";
    pub const CUSTOMER_PHONE: &str = "0123456789";

    lazy_static! {
        pub static ref NEW_CUSTOMER: NewCustomer = NewCustomer {
            name: CUSTOMER_NAME.to_string(),
            surname: CUSTOMER_SURNAME.to_string(),
            email: Some(CUSTOMER_EMAIL.to_string()),
            phone: Some(CUSTOMER_PHONE.to_string())
        };
    }
}
