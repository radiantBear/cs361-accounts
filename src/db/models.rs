use diesel::prelude::*;

use super::schema;


#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::users)]
#[diesel(primary_key(user_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: Vec<u8>,
    pub date_created: chrono::NaiveDateTime,
    pub date_updated: chrono::NaiveDateTime
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(primary_key(session_id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::sessions)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Session {
    pub session_id: i32,
    pub uuid: String,
    pub user_id: i32,
    pub date_created: chrono::NaiveDateTime,
    pub date_expires: chrono::NaiveDateTime
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(primary_key(nonce_id))]
#[diesel(table_name = schema::nonces)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Nonce {
    pub nonce_id: i32,
    pub uuid: String,
    pub date_created: chrono::NaiveDateTime
}


#[derive(Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewUser<'a> {
    pub username: String,
    pub password: &'a [u8]
}

#[derive(Insertable)]
#[diesel(table_name = schema::sessions)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewSession {
    pub uuid: String,
    pub user_id: i32
}

#[derive(Insertable)]
#[diesel(table_name = schema::nonces)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewNonce {
    pub uuid: String
}