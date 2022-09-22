// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (hash_id) {
        hash_id -> Text,
        created_ts -> Integer,
        updated_ts -> Integer,
    }
}
