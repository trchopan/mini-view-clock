// @generated automatically by Diesel CLI.

diesel::table! {
    plex_hook_token (id) {
        id -> Nullable<Integer>,
        token -> Text,
        created -> Text,
    }
}
