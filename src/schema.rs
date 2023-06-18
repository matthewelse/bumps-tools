// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Integer,
        club -> Text,
        crew -> Text,
        year -> Integer,
        day -> Integer,
        position -> Integer,
        competition -> Integer,
    }
}
