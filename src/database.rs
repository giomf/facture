use std::path::Path;

use crate::models::MODELS;
use anyhow::{anyhow, Result};
use native_db::{Builder, Database, ToInput};

pub const DATABASE_PATH: &str = "./facture.db";

pub struct FactureDatabase<'a> {
    builder: Builder,
    database: Database<'a>,
}

impl<'a> FactureDatabase<'a> {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let builder = Builder::new();
        let database = builder.create(&MODELS, path.as_ref())?;
        let database = Self { builder, database };
        Ok(database)
    }

    pub fn open_in_memory() -> Result<Self> {
        let builder = Builder::new();
        let database = builder.create_in_memory(&MODELS)?;
        let database = Self { builder, database };
        Ok(database)
    }

    pub fn exists<T: ToInput>(&self, uuid: &str) -> Result<bool> {
        let r = self.database.r_transaction()?;
        let result: Option<T> = r.get().primary(uuid)?;
        Ok(result.is_some())
    }

    pub fn insert<T: ToInput + Clone>(&self, item: &T) -> Result<()> {
        let rw = self.database.rw_transaction()?;
        rw.insert(item.to_owned())?;
        rw.commit()?;
        Ok(())
    }

    pub fn update<T: ToInput + Clone>(&self, uuid: &str, item: &T) -> Result<()> {
        let rw = self.database.rw_transaction()?;
        let old: T = self
            .read(uuid)?
            .ok_or_else(|| anyhow!("{uuid} not found"))?;

        rw.update(old, item.to_owned())?;
        rw.commit()?;
        Ok(())
    }

    pub fn read<T: ToInput>(&self, uuid: &str) -> Result<Option<T>> {
        let r = self.database.r_transaction()?;
        let result: Option<T> = r.get().primary(uuid)?;
        Ok(result)
    }

    pub fn read_all<T: ToInput>(&self) -> Result<Vec<T>> {
        let r = self.database.r_transaction()?;
        let result = r.scan().primary()?.all()?.filter_map(Result::ok).collect();
        Ok(result)
    }

    pub fn remove<T: ToInput>(&self, uuid: &str) -> Result<()> {
        let rw = self.database.rw_transaction()?;
        let item: T = self
            .read(uuid)?
            .ok_or_else(|| anyhow!("{uuid} not found"))?;
        rw.remove(item)?;
        rw.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Customer;
    use std::sync::LazyLock;

    // Constants for test data
    const CUSTOMER_1_NAME: &str = "John";
    const CUSTOMER_1_SURNAME: &str = "Doe";
    const CUSTOMER_1_EMAIL: &str = "john.doe@example.com";
    const CUSTOMER_1_PHONE: &str = "123-456-7890";
    const CUSTOMER_2_NAME: &str = "Jane";
    const CUSTOMER_2_EMAIL: &str = "jane.doe@example.com";
    const CUSTOMER_2_PHONE: &str = "987-654-3210";

    static CUSTOMER_1: LazyLock<Customer> = LazyLock::new(|| {
        Customer::builder()
            .name(CUSTOMER_1_NAME)
            .surname(CUSTOMER_1_SURNAME)
            .phone(CUSTOMER_1_PHONE)
            .email(CUSTOMER_1_EMAIL)
            .build()
    });
    static CUSTOMER_2: LazyLock<Customer> = LazyLock::new(|| {
        Customer::builder()
            .name(CUSTOMER_2_NAME)
            .surname(CUSTOMER_1_SURNAME)
            .phone(CUSTOMER_2_PHONE)
            .email(CUSTOMER_2_EMAIL)
            .build()
    });

    #[test]
    fn test_insert() -> Result<()> {
        let database = FactureDatabase::open_in_memory()?;

        let insert_result = database.insert(&*CUSTOMER_1);
        assert!(insert_result.is_ok(), "Failed to insert customer");

        let read_customer: Customer = database.read(&CUSTOMER_1.uuid)?.unwrap();
        assert_eq!(*CUSTOMER_1, read_customer);
        Ok(())
    }

    #[test]
    fn test_update() -> Result<()> {
        let database = FactureDatabase::open_in_memory()?;
        database.insert(&*CUSTOMER_1)?;

        let updated_customer = Customer::builder()
            .uuid(&CUSTOMER_1.uuid)
            .name(CUSTOMER_2_NAME)
            .surname(CUSTOMER_1_SURNAME)
            .email(CUSTOMER_2_EMAIL)
            .phone(CUSTOMER_2_PHONE)
            .build();

        database.update(&CUSTOMER_1.uuid, &updated_customer)?;
        let read_customer: Customer = database.read(&CUSTOMER_1.uuid)?.unwrap();

        assert_eq!(read_customer.name, CUSTOMER_2_NAME);
        assert_eq!(read_customer.surname, CUSTOMER_1_SURNAME);
        assert_eq!(read_customer.email.unwrap(), CUSTOMER_2_EMAIL);
        assert_eq!(read_customer.phone.unwrap(), CUSTOMER_2_PHONE);
        Ok(())
    }

    #[test]
    fn test_exists() -> Result<()> {
        let database = FactureDatabase::open_in_memory()?;

        // Insert the customer first
        database.insert(&*CUSTOMER_1)?;

        // Check if the customer exists
        let exists = database.exists::<Customer>(&CUSTOMER_1.uuid)?;
        assert!(exists, "Customer should exist in the database");

        // Check for a non-existing customer
        let non_existent = database.exists::<Customer>("non-existent-uuid")?;
        assert!(!non_existent, "Customer should not exist in the database");
        Ok(())
    }

    #[test]
    fn test_read_all() -> Result<()> {
        let database = FactureDatabase::open_in_memory()?;

        database.insert(&*CUSTOMER_1)?;
        database.insert(&*CUSTOMER_2)?;
        let all_customers: Vec<Customer> = database.read_all()?;

        assert_eq!(all_customers.len(), 2);
        assert!(all_customers.iter().any(|c| c.uuid == CUSTOMER_1.uuid));
        assert!(all_customers.iter().any(|c| c.uuid == CUSTOMER_2.uuid));

        Ok(())
    }

    #[test]
    fn test_remove() -> Result<()> {
        let database = FactureDatabase::open_in_memory()?;
        database.insert(&*CUSTOMER_1)?;
        database.remove::<Customer>(&CUSTOMER_1.uuid)?;
        assert!(!database.exists::<Customer>(&CUSTOMER_1.uuid)?);
        Ok(())
    }
}
