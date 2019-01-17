table! {
    post (id) {
        id -> Integer,
        author -> Integer,
        date -> Timestamp,
        content -> Text,
        title -> Text,
        excerpt -> Text,
        status -> Text,
        comment_status -> Text,
        slug -> Text,
    }
}

table! {
    comment (id) {
        id -> Integer,
        date -> Timestamp,
        content -> Text,
        status -> Text,
        post_id -> Integer,
        author_name -> Text,
        author_mail -> Nullable<Text>,
        author_url -> Nullable<Text>,
        author_useragent -> Nullable<Text>,
    }
}

table! {
    username (id) {
        id -> Integer,
        password -> Text,
        email -> Text,
        display_name -> Text,
    }
}

joinable!(post -> username (author));
joinable!(comment -> post (post_id));
allow_tables_to_appear_in_same_query!(post, username);
allow_tables_to_appear_in_same_query!(post, comment);