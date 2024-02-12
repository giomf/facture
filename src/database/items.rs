use crate::database::{schema::items, Repository};
use diesel::sqlite::SqliteConnection;
use diesel::{dsl::exists, prelude::*, select};

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub id: i32,
    pub position: i32,
    pub name: String,
    pub amount: i32,
    pub price: f32,
    pub invoice_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewItem {
    pub position: i32,
    pub name: String,
    pub amount: i32,
    pub price: f32,
    pub invoice_id: i32,
}

#[derive(AsChangeset, Default, Debug)]
#[diesel(table_name = items)]
pub struct UpdateItem {
    pub position: Option<i32>,
    pub name: Option<String>,
    pub amount: Option<i32>,
    pub price: Option<f32>,
    pub invoice_id: Option<i32>,
}
pub struct ItemRepository {
    connection: SqliteConnection,
}

impl Repository<Item, NewItem, UpdateItem> for ItemRepository {
    fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    fn create(&mut self, new_item: &NewItem) -> anyhow::Result<Item> {
        use crate::database::schema::items::dsl::*;

        let new_item: Item = diesel::insert_into(items)
            .values(new_item)
            .get_result(&mut self.connection)?;

        Ok(new_item)
    }

    fn exists(&mut self, item_id: i32) -> anyhow::Result<bool> {
        use crate::database::schema::items::dsl::*;

        let result =
            select(exists(items.filter(id.eq(item_id)))).get_result(&mut self.connection)?;

        Ok(result)
    }

    fn read(&mut self, item_id: i32) -> anyhow::Result<Option<Item>> {
        use crate::database::schema::items::dsl::*;

        let item = items
            .find(item_id)
            .first::<Item>(&mut self.connection)
            .optional()?;

        Ok(item)
    }

    fn read_all(&mut self) -> anyhow::Result<Vec<Item>> {
        use crate::database::schema::items::dsl::*;

        let all_items = items.get_results::<Item>(&mut self.connection)?;

        Ok(all_items)
    }

    fn update(&mut self, item_id: i32, update_item: &UpdateItem) -> anyhow::Result<Item> {
        use crate::database::schema::items::dsl::*;

        let update_item: Item = diesel::update(items)
            .filter(id.eq(item_id))
            .set(update_item)
            .get_result(&mut self.connection)?;

        Ok(update_item)
    }

    fn delete(&mut self, item_id: i32) -> anyhow::Result<Item> {
        use crate::database::schema::items::dsl::*;

        let deleted_item: Item = diesel::delete(items)
            .filter(id.eq(item_id))
            .get_result(&mut self.connection)?;

        Ok(deleted_item)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::database::{
        create_connection, customer::CustomerRepository, invoice::InvoiceRepository, tests::*,
    };

    fn setup() -> anyhow::Result<(
        TempDir,
        CustomerRepository,
        InvoiceRepository,
        ItemRepository,
    )> {
        let (temp_dir, database_path) = init_database()?;
        let customers_connection = create_connection(&database_path);
        let invoices_connection = create_connection(&database_path);
        let items_connection = create_connection(&database_path);
        let customers = CustomerRepository::new(customers_connection);
        let invoices = InvoiceRepository::new(invoices_connection);
        let items = ItemRepository::new(items_connection);
        Ok((temp_dir, customers, invoices, items))
    }

    #[test]
    fn create() -> anyhow::Result<()> {
        let (_temp_dir, mut customers, mut invoices, mut items) = setup()?;
        let customer = init_customer(&mut customers)?;
        let invoice = init_invoice(&customer, &mut invoices)?;
        let new_item = NewItem {
            position: ITEM_POSITION,
            name: ITEM_NAME.to_string(),
            amount: ITEM_AMOUNT,
            price: ITEM_PRICE,
            invoice_id: invoice.id,
        };
        let result = items.create(&new_item);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn read() -> anyhow::Result<()> {
        let (_temp_dir, mut customers, mut invoices, mut items) = setup()?;
        let customer = init_customer(&mut customers)?;
        let invoice = init_invoice(&customer, &mut invoices)?;
        let item = init_item(&invoice, &mut items)?;
        let result = items.read(item.id);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn read_all() -> anyhow::Result<()> {
        let (_temp_dir, mut customers, mut invoices, mut items) = setup()?;
        let customer = init_customer(&mut customers)?;
        let invoice = init_invoice(&customer, &mut invoices)?;
        let mut created_items = Vec::new();
        for _ in 0..2 {
            created_items.push(init_item(&invoice, &mut items)?);
        }
        let all_items = items.read_all()?;
        assert_eq!(created_items, all_items);
        Ok(())
    }

    #[test]
    fn delete() -> anyhow::Result<()> {
        let (_temp_dir, mut customers, mut invoices, mut items) = setup()?;
        let customer = init_customer(&mut customers)?;
        let invoice = init_invoice(&customer, &mut invoices)?;
        let item = init_item(&invoice, &mut items)?;
        let result = items.delete(item.id);
        assert!(result.is_ok());
        Ok(())
    }
}
