pub mod customer;
pub mod schema;

use diesel::{sqlite::SqliteConnection, Connection};

pub const DATABASE_PATH: &str = "./facture.sqlite";

pub trait Repository<T, E> {
    fn new(connection: SqliteConnection) -> Self;
    fn create(&mut self, element: &E) -> anyhow::Result<T>;
    fn read(&mut self, id: i32) -> anyhow::Result<Option<T>>;
    fn read_all(&mut self) -> anyhow::Result<Vec<T>>;
    fn delete(&mut self, id: i32) -> anyhow::Result<T>;
}

pub fn create_connection(database_path: &str) -> SqliteConnection {
    SqliteConnection::establish(database_path)
        .expect(format!("Unable to open {}", database_path).as_str())
}
