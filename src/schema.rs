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