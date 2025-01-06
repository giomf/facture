use super::RenderAble;
use crate::database::{
    models::{Business, Customer, Invoice, Item},
    YamlAble,
};
use serde::{Deserialize, Serialize};

const TEMPLATE_URL: &str =
    "https://raw.githubusercontent.com/erictapen/typst-invoice/refs/heads/main/lib.typ";
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
#let data = (
  invoice-nr: data.invoice-nr,
  invoice-date: parse-date(data.invoice-date),
  items: data.items,
  author: (
    name: data.author.name,
    street: data.author.street,
    zip: data.author.zip,
    city: data.author.city,
    tax_nr: data.author.tax_nr,
    signature: if data.author.signature == none {
      none
    } else {
      image(data.author.signature, width: 5em)
    }
  ),
  recipient: data.recipient,
  bank-account: data.bank-account,
  vat: data.vat,
  kleinunternehmer: data.kleinunternehmer,
)

#show: invoice(
  data.invoice-nr,
  data.invoice-date,
  data.items,
  data.author,
  data.recipient,
  data.bank-account,
  vat: data.vat,
  kleinunternehmer: data.kleinunternehmer,
)
 "#;

pub mod template {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Invoice {
        #[serde(rename = "invoice-nr")]
        pub invoice_nr: String,
        #[serde(rename = "invoice-date")]
        pub invoice_date: String,
        pub kleinunternehmer: bool,
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
        pub zip: String,
        pub city: String,
        pub tax_nr: String,
        pub signature: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Recipient {
        pub name: String,
        pub street: String,
        pub zip: String,
        pub city: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct BankAccount {
        pub name: String,
        pub bank: String,
        pub iban: String,
        pub bic: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gender: Option<Gender>,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Gender {
        pub account_holder: String,
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
            invoice_nr: invoice.id,
            invoice_date: invoice.issuing_date.to_string(),
            kleinunternehmer: business.small_business,
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
            zip: business.address.postal_code,
            city: business.address.city,
            tax_nr: business.tax_number,
            signature: None,
        }
    }
}

impl From<Customer> for template::Recipient {
    fn from(customer: Customer) -> Self {
        Self {
            name: format!("{} {}", customer.contact.name, customer.contact.surname),
            street: format!("{} {}", customer.address.street, customer.address.number),
            zip: customer.address.postal_code,
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
