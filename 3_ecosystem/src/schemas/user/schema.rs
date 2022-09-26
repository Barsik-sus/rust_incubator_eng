// @generated automatically by Diesel CLI.

diesel::table! {
    friendship (user_id, friend_id) {
        user_id -> Int4,
        friend_id -> Int4,
    }
}

diesel::table! {
    user_password (user_id) {
        user_id -> Int4,
        password -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(user_password -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    friendship,
    user_password,
    users,
);
