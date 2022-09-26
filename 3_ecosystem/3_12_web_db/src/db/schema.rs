// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
    }
}

diesel::table! {
    labels (id) {
        id -> Integer,
        label -> Text,
        article_id -> Integer,
    }
}

diesel::joinable!(labels -> articles (article_id));

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    labels,
);
