// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
