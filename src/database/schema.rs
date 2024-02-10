// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Integer,
        name -> Text,
        surname -> Text,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
    }
}

diesel::table! {
    invoices (id) {
        id -> Integer,
        customer_id -> Integer,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        position -> Integer,
        name -> Text,
        amount -> Integer,
        price -> Float,
        invoice_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    invoices,
    items,
);
