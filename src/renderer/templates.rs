use crate::database::{
    models::{
        business::Business,
        customer::Customer,
        invoice::{Invoice, Item},
        Address,
    },
    YamlAble,
};
use chrono::Days;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TemplateInvoice {
    #[serde(rename = "invoice-id")]
    pub invoice_id: String,
    #[serde(rename = "delivery-date")]
    pub delivery_date: String,
    #[serde(rename = "issuing-date")]
    pub issuing_date: String,
    #[serde(rename = "due-date")]
    pub due_date: String,
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<TemplateDiscount>,
    pub vat: f32,
    #[serde(rename = "hourly-rate")]
    pub hourly_rate: u32,
    pub biller: TemplateBiller,
    pub recipient: TemplateRecipient,
    pub items: Vec<TemplateItem>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TemplateDiscount {
    pub value: f64,
    pub r#type: String,
    pub reason: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TemplateBiller {
    pub name: String,
    pub website: String,
    pub job: String,
    #[serde(rename = "vat-id")]
    pub vat_id: String,
    #[serde(rename = "umsatzsteuer-identifikationsnummer")]
    pub umsatzsteuer_identifikationsnummer: String,
    pub iban: String,
    #[serde(rename = "smallBusiness")]
    pub small_business: bool,
    pub paypalme: String,
    pub address: TemplateAddress,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TemplateRecipient {
    pub name: String,
    pub organization: String,
    #[serde(rename = "vat-id")]
    pub vat_id: String,
    pub address: TemplateAddress,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TemplateAddress {
    pub country: String,
    pub city: String,
    #[serde(rename = "postal-code")]
    pub postal_code: String,
    pub street: String,
    pub number: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TemplateItem {
    pub date: String,
    pub description: String,
    pub price: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    #[serde(
        rename = "dur-min",
        skip_serializing_if = "Option::is_none",
        default = "duration_default"
    )]
    pub dur_min: Option<u32>,
}

fn duration_default() -> Option<u32> {
    Some(1)
}

impl YamlAble for TemplateInvoice {}

impl TemplateInvoice {
    pub fn new(business: Business, customer: Customer, invoice: Invoice) -> Self {
        Self {
            invoice_id: invoice.id,
            delivery_date: invoice.delivery_date.to_string(),
            issuing_date: invoice.issuing_date.to_string(),
            due_date: invoice
                .issuing_date
                .checked_add_days(Days::new(15))
                .unwrap()
                .to_string(),
            language: "de".to_owned(),
            vat: business.vat,
            biller: business.into(),
            recipient: customer.into(),
            items: invoice.items.into_iter().map(|item| item.into()).collect(),
            ..Default::default()
        }
    }
}

impl From<Business> for TemplateBiller {
    fn from(business: Business) -> Self {
        Self {
            name: format!("{} {}", business.contact.name, business.contact.surname),
            job: business.organisation,
            address: business.address.into(),
            iban: business.payment.iban,
            vat_id: business.vat_id,
            ..Default::default()
        }
    }
}

impl From<Address> for TemplateAddress {
    fn from(address: Address) -> Self {
        Self {
            country: address.country,
            city: address.city,
            postal_code: address.postal_code,
            street: address.street,
            number: address.number,
        }
    }
}

impl From<Customer> for TemplateRecipient {
    fn from(customer: Customer) -> Self {
        Self {
            name: format!("{} {}", customer.contact.name, customer.contact.surname),
            organization: customer.organization,
            address: customer.address.into(),
            vat_id: customer.vat_id,
        }
    }
}

impl From<Item> for TemplateItem {
    fn from(item: Item) -> Self {
        Self {
            description: item.description,
            price: item.price,
            quantity: item.quantity,
            ..Default::default()
        }
    }
}
