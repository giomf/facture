use super::RenderAble;
use crate::database::{
    models::{Business, Customer, Invoice, Item},
    YamlAble,
};
use serde::{Deserialize, Serialize};

const TEMPLATE_URL: &str =
    "https://raw.githubusercontent.com/giomf/facture-template/refs/heads/main/lib.typ";

const TEMPLATE_MAIN_CONTENT: &str = r#"
#import "template.typ": invoice

#let parse-date = (date-str) => {
  let parts = date-str.split("-")
  if parts.len() != 3 {
    panic(
      "Invalid date string: " + date-str + "\n" +
      "Expected format: YYYY-MM-DD"
    )
  }
  datetime(
    year: int(parts.at(0)),
    month: int(parts.at(1)),
    day: int(parts.at(2)),
  )
}

#let data = yaml("data.yaml")

#show: invoice(
  data.id,
  issuing-date: parse-date(data.issuing-date),
  data.items,
  data.author,
  data.recipient,
  data.bank-account,
  due-days: data.due-days,
  service-date: parse-date(data.service-date),
  vat: data.vat,
  small-business: data.small-business,
)
 "#;

pub mod template {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Invoice {
        pub id: String,
        #[serde(rename = "issuing-date")]
        pub issuing_date: String,
        #[serde(rename = "service-date")]
        pub service_date: String,
        #[serde(rename = "small-business")]
        pub small_business: bool,
        #[serde(rename = "due-days")]
        pub due_days: u32,
        pub vat: f32,
        pub author: Author,
        pub recipient: Recipient,
        #[serde(rename = "bank-account")]
        pub bank_account: BankAccount,
        pub items: Vec<Item>,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Author {
        pub name: String,
        pub street: String,
        #[serde(rename = "postal-code")]
        pub postal_code: String,
        pub city: String,
        #[serde(rename = "tax-number")]
        pub tax_number: String,
        pub signature: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Recipient {
        pub name: String,
        pub street: String,
        #[serde(rename = "postal-code")]
        pub postal_code: String,
        pub city: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct BankAccount {
        pub name: String,
        pub bank: String,
        pub iban: String,
        pub bic: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Item {
        pub description: String,
        pub price: f32,
    }
}

impl YamlAble for template::Invoice {}
impl RenderAble for template::Invoice {
    fn new(business: Business, customer: Customer, invoice: Invoice) -> Self {
        Self {
            id: invoice.id,
            issuing_date: invoice.issuing_date.to_string(),
            service_date: invoice.delivery_date.to_string(),
            small_business: business.small_business,
            due_days: invoice.due_days,
            vat: business.vat,
            author: business.clone().into(),
            recipient: customer.into(),
            bank_account: business.into(),
            items: invoice.items.into_iter().map(|item| item.into()).collect(),
        }
    }

    fn main() -> String {
        TEMPLATE_MAIN_CONTENT.to_owned()
    }

    fn url() -> String {
        TEMPLATE_URL.to_owned()
    }
}

impl From<Business> for template::Author {
    fn from(business: Business) -> Self {
        Self {
            name: format!("{} {}", business.contact.name, business.contact.surname),
            street: format!("{} {}", business.address.street, business.address.number),
            postal_code: business.address.postal_code,
            city: business.address.city,
            tax_number: business.tax_number,
            signature: None,
        }
    }
}

impl From<Customer> for template::Recipient {
    fn from(customer: Customer) -> Self {
        Self {
            name: format!("{} {}", customer.contact.name, customer.contact.surname),
            street: format!("{} {}", customer.address.street, customer.address.number),
            postal_code: customer.address.postal_code,
            city: customer.address.city,
        }
    }
}

impl From<Business> for template::BankAccount {
    fn from(business: Business) -> Self {
        Self {
            name: format!("{} {}", business.contact.name, business.contact.surname),
            bank: business.payment.bank,
            iban: business.payment.iban,
            bic: business.payment.bic,
            ..Default::default()
        }
    }
}

impl From<Item> for template::Item {
    fn from(item: Item) -> Self {
        Self {
            description: item.description,
            price: item.price,
        }
    }
}
