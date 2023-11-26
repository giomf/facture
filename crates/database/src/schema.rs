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
