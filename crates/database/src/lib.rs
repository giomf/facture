pub mod customer;
pub mod schema;

use diesel::sqlite::SqliteConnection;

pub trait Repository<T, E> {
    fn new(connection: SqliteConnection) -> Self;
    fn read(&mut self, id: i32) -> anyhow::Result<Option<T>>;
    fn create(&mut self, element: &E) -> anyhow::Result<()>;
    fn delete(&mut self, id: i32) -> anyhow::Result<()>;
}
