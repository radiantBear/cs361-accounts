use diesel::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: Vec<u8>,
    pub date_created: chrono::NaiveDateTime,
    pub date_updated: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a [u8]
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Session {
    pub session_id: i32,
    pub uuid: String,
    pub user_id: i32,
    pub date_created: chrono::NaiveDateTime,
    pub date_expires: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewSession {
    pub uuid: String,
    pub user_id: i32
}