// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        username -> Text,
        password -> Text,
    }
}
