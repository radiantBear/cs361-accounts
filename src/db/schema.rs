diesel::table! {
    users (user_id) {
        user_id -> Integer,
        username -> VarChar,
        password -> Binary,
        date_created -> Datetime,
        date_updated -> Datetime
    }
}

diesel::table! {
    sessions (session_id) {
        session_id -> Integer,
        uuid -> Char,
        user_id -> Integer,
        date_created -> Datetime,
        date_expires -> Datetime
    }
}

diesel::table! {
    nonces (nonce_id) {
        nonce_id -> Integer,
        uuid -> Char,
        date_created -> Datetime
    }
}

diesel::joinable!(sessions -> users (user_id));
diesel::allow_tables_to_appear_in_same_query!(sessions, users);