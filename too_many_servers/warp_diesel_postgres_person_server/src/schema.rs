// @generated automatically by Diesel CLI.

diesel::table! {
    person (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        created -> Timestamptz,
    }
}
