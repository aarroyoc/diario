table! {
    post (id) {
        id -> Integer,
        author -> Integer,
        date -> Timestamp,
        content -> Text,
        title -> Text,
        excerpt -> Text,
        status -> VarChar,
        comment_status -> VarChar,
        name -> Text,
        modified -> Timestamp,
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
allow_tables_to_appear_in_same_query!(post, username);