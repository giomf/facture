pub mod schema;
pub mod customer;

static DATABASE_URL: &str = "./facture.sqlite";

pub trait Repository<T,E> {
    fn new() -> Self;
    fn read(&mut self, id: i32) -> anyhow::Result<Option<T>>;
    fn create(&mut self, element: &E) -> anyhow::Result<()>;
    fn delete(&mut self, id: i32) -> anyhow::Result<()>;
}

