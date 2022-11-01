// @generated automatically by Diesel CLI.

diesel::table! {
    pathway (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
