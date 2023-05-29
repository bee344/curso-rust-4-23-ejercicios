// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
);
