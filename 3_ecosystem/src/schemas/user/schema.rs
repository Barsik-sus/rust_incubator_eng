// @generated automatically by Diesel CLI.

diesel::table! {
    friendship (id) {
        id -> Int4,
        user_id -> Int4,
        friend_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    friendship,
    users,
);
