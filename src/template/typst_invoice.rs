use super::TemplateAble;
use crate::database::{
    models::{business, customer, invoice},
    YamlAble,
};
use serde::{Deserialize, Serialize};

const TEMPLATE_URL: &str = "https://github.com/giomf/typst-invoice/blob/add-data-yaml/lib.typ";
const TEMPLATE_MAIN_CONTENT: &str = r#"
#import "template.typ": invoice

#show: invoice(
  data: yaml("data.yaml")
)
"#;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TypstInvoice {
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

impl YamlAble for TypstInvoice {}

impl TemplateAble for TypstInvoice {
    fn new(
        business: business::Business,
        customer: customer::Customer,
        invoice: invoice::Invoice,
    ) -> Self {
        Self {
            invoice_nr: invoice.id,
            invoice_date: invoice.issuing_date.to_string(),
            kleinunternehmer: false,
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

impl From<business::Business> for Author {
    fn from(business: business::Business) -> Self {
        Self {
            name: format!("{} {}", business.contact.name, business.contact.surname),
            street: format!("{} {}", business.address.street, business.address.number),
            zip: business.address.postal_code,
            city: business.address.city,
            tax_nr: business.vat_id,
            signature: None,
        }
    }
}

impl From<customer::Customer> for Recipient {
    fn from(customer: customer::Customer) -> Self {
        Self {
            name: format!("{} {}", customer.contact.name, customer.contact.surname),
            street: format!("{} {}", customer.address.street, customer.address.number),
            zip: customer.address.postal_code,
            city: customer.address.city,
        }
    }
}

impl From<business::Business> for BankAccount {
    fn from(business: business::Business) -> Self {
        Self {
            name: format!("{} {}", business.contact.name, business.contact.surname),
            bank: business.payment.bank,
            iban: business.payment.iban,
            ..Default::default()
        }
    }
}

impl From<invoice::Item> for Item {
    fn from(item: invoice::Item) -> Self {
        Self {
            description: item.description,
            price: item.price,
        }
    }
}
